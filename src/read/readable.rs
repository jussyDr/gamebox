use std::{
    io::{BufRead, Cursor, Read, Seek},
    path::PathBuf,
    rc::Rc,
};

use lzo::lzo1x;
use serde_jsonrc::Value;

use crate::{
    common::{
        Class, Compression, FileFormat, GAMEBOX_FILE_SIGNATURE, GAMEBOX_VERSION, NODE_END, SKIP,
        UNKNOWN_BYTE,
    },
    deserialize::{Deserializer, IdState, IdStateRef, NodeRef, NodeState, NodeStateRef, Take},
};

use super::{BodyOptions, HeaderOptions, Result};

pub fn read_gbx<T: Default + Class + HeaderChunks + ReadBody>(
    reader: impl BufRead + Seek,
    header_options: HeaderOptions,
    body_options: BodyOptions,
) -> Result<T> {
    let mut node = T::default();

    let mut d = Deserializer::new(reader, (), ());

    if d.byte_array()? != GAMEBOX_FILE_SIGNATURE {
        return Err("not a gamebox file".into());
    }

    if d.u16()? != GAMEBOX_VERSION {
        return Err("unsupported gamebox version".into());
    }

    let format = FileFormat::read(&mut d)?;

    if let FileFormat::Text = format {
        return Err("text format is not supported".into());
    }

    let ref_table_compression = Compression::read(&mut d)?;

    if let Compression::Compressed = ref_table_compression {
        return Err("compressed reference table is not supported".into());
    }

    let body_compression = Compression::read(&mut d)?;

    if d.u8()? != UNKNOWN_BYTE {
        return Err("invalid unknown byte".into());
    }

    let class_id = d.u32()?;

    if class_id != T::CLASS_ID.get() {
        return Err("class id does not match".into());
    }

    let header_data_size = d.u32()?;

    match header_options {
        HeaderOptions::Read { read_heavy_chunks } => {
            read_header(
                &mut node,
                d.take_with(header_data_size as u64, (), ()),
                read_heavy_chunks,
            )?;
        }
        HeaderOptions::Skip { assume_size_zero } => {
            if !assume_size_zero {
                d.skip(header_data_size)?;
            }
        }
    }

    let num_nodes = d.u32()?;

    let node_state = NodeState::new(num_nodes as usize);

    let num_node_refs = d.u32()?;

    if num_node_refs > 0 {
        d.u32()?;
        let mut folders = vec![];
        read_folders(&mut d, PathBuf::new(), &mut folders)?;

        d.repeat(num_node_refs as usize, |d| {
            d.u32()?;
            let file_name = d.string()?;
            let node_index = d.u32()?;
            d.u32()?;
            let folder_index = d.u32()?;

            let mut file_path = folders[folder_index as usize].clone();
            file_path.push(file_name);

            node_state.set(
                node_index as usize,
                NodeRef::External {
                    path: Rc::from(file_path),
                },
            )?;

            Ok(())
        })?;
    }

    match body_compression {
        Compression::Compressed => {
            let body_size = d.u32()?;
            let compressed_body_size = d.u32()?;

            match body_options {
                BodyOptions::Read { .. } => {
                    let compressed_body = d.bytes(compressed_body_size as usize)?;

                    let mut buf = vec![0; body_size as usize];

                    let body = lzo1x::decompress_safe(&compressed_body, &mut buf).unwrap();

                    let reader = Cursor::new(body);

                    let mut d = Deserializer::new(reader, IdState::new(), node_state);

                    T::read_body(&mut node, &mut d)?;

                    d.eof()?;
                }
                BodyOptions::Skip => {
                    d.skip(compressed_body_size)?;
                }
            }

            d.eof()?;
        }
        Compression::Uncompressed => match body_options {
            BodyOptions::Read { .. } => {
                let reader = d.into_reader();

                let mut d = Deserializer::new(reader, IdState::new(), node_state);

                T::read_body(&mut node, &mut d)?;

                d.eof()?;
            }
            BodyOptions::Skip => {}
        },
    }

    Ok(node)
}

fn read_header<T: HeaderChunks, R: BufRead + Seek, I, N>(
    node: &mut T,
    mut d: Deserializer<R, I, N>,
    read_heavy_chunks: bool,
) -> Result<()> {
    let header_chunks = d.list(|d| {
        let chunk_id = d.u32()?;
        let chunk_size = d.u32()?;

        Ok((chunk_id, chunk_size))
    })?;

    let id_state = IdState::new();

    let mut header_chunk_entries = T::header_chunks();

    for (chunk_id, chunk_size) in header_chunks {
        let is_heavy_chunk = chunk_size & 0x80000000 != 0;
        let chunk_size = chunk_size & 0x7FFFFFFF;

        if is_heavy_chunk && !read_heavy_chunks {
            d.skip(chunk_size)?;
        } else {
            let mut d = d.take_with(chunk_size as u64, &id_state, ());

            let header_chunk_entry = header_chunk_entries
                .find(|header_chunk_entry| header_chunk_entry.id == chunk_id)
                .ok_or("unknown header chunk")?;

            (header_chunk_entry.read_fn)(node, &mut d)?;

            d.eof()?;
        }
    }

    d.eof()?;

    Ok(())
}

fn read_folders<R: Read, I, N>(
    d: &mut Deserializer<R, I, N>,
    path: PathBuf,
    folders: &mut Vec<PathBuf>,
) -> Result<()> {
    folders.push(path.clone());

    d.list(|d| {
        let folder_name = d.string()?;

        let mut path = path.clone();
        path.push(folder_name);

        read_folders(d, path, folders)
    })?;

    Ok(())
}

pub fn read_body_chunks<T: BodyChunks, R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    node: &mut T,
    d: &mut Deserializer<R, I, N>,
) -> Result<()> {
    let mut body_chunk_entries = T::body_chunks();

    loop {
        let chunk_id = d.u32()?;

        if chunk_id == NODE_END {
            break;
        }

        let body_chunk_entry = body_chunk_entries
            .find(|body_chunk_entry| body_chunk_entry.id == chunk_id)
            .ok_or("unknown body chunk")?;

        match body_chunk_entry.read_fn {
            BodyChunkReadFn::Normal(read_fn) => {
                read_fn(node, d)?;
            }
            BodyChunkReadFn::Skippable(read_fn) => {
                if d.u32()? != SKIP {
                    return Err("expected skippable chunk".into());
                }

                let chunk_size = d.u32()?;

                let mut d = d.take(chunk_size as u64);

                read_fn(node, &mut d)?;

                d.eof()?;
            }
        }
    }

    Ok(())
}

pub struct HeaderChunkEntry<T, R> {
    pub id: u32,
    pub read_fn: HeaderChunkReadFn<T, R>,
}

type HeaderChunkReadFn<T, R> =
    fn(n: &mut T, d: &mut Deserializer<Take<&mut R>, &IdState, ()>) -> Result<()>;

pub struct BodyChunkEntry<T, R, I, N> {
    pub id: u32,
    pub read_fn: BodyChunkReadFn<T, R, I, N>,
}

pub enum BodyChunkReadFn<T, R, I, N> {
    Normal(NormalBodyChunkReadFn<T, R, I, N>),
    Skippable(SkippableBodyChunkReadFn<T, R, I, N>),
}

pub type NormalBodyChunkReadFn<T, R, I, N> =
    fn(n: &mut T, d: &mut Deserializer<R, I, N>) -> Result<()>;

pub type SkippableBodyChunkReadFn<T, R, I, N> =
    fn(n: &mut T, d: &mut Deserializer<Take<&mut R>, &I, &N>) -> Result<()>;

pub trait Sealed {
    fn read(
        reader: impl BufRead + Seek,
        header_options: HeaderOptions,
        body_options: BodyOptions,
    ) -> Result<Self>
    where
        Self: Sized;
}

pub trait HeaderChunks {
    fn header_chunks<R: BufRead>() -> impl Iterator<Item = HeaderChunkEntry<Self, R>>
    where
        Self: Sized;
}

pub trait BodyChunks {
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>>
    where
        Self: Sized;
}

pub trait ReadBody {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()>;
}

pub trait ReadJson {
    const CLASS_NAME: &'static str;

    fn read(json: Value) -> Result<Self>
    where
        Self: Sized;
}

pub fn read_json<T: ReadJson>(reader: impl Read) -> Result<T> {
    let mut value: Value = serde_jsonrc::from_reader(reader).unwrap();
    let object = value.as_object_mut().unwrap();
    let class_name = object.get("ClassId").unwrap();

    if class_name != T::CLASS_NAME {
        return Err("class name does not match".into());
    }

    object.remove("ClassId");

    T::read(value)
}
