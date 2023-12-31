use std::{
    io::{BufRead, Cursor, Read, Seek},
    path::PathBuf,
};

use serde_jsonrc::Value;

use crate::common::{ClassId, FILE_SIGNATURE, NODE_END, SKIP};

use super::{
    deserialize::{Deserializer, IdState, IdStateMut, NodeState, NodeStateMut, Take},
    BodyOptions, HeaderOptions, Result,
};

pub fn read_gbx<T: Default + ClassId + HeaderChunks + ReadBody>(
    reader: impl BufRead + Seek,
    header_options: HeaderOptions,
    body_options: BodyOptions,
) -> Result<T> {
    let mut node = T::default();

    let mut d = Deserializer::new(reader, (), ());

    if d.byte_array()? != FILE_SIGNATURE {
        return Err("invalid file signature".into());
    }

    if d.u16()? != 6 {
        return Err("unsupported gbx version".into());
    }

    if d.u8()? != b'B' {
        return Err("unsupported gbx format".into());
    }

    if d.u8()? != b'U' {
        return Err("unsupported reference table compression".into());
    }

    let is_body_compressed = match d.u8()? {
        b'C' => true,
        b'U' => false,
        _ => return Err("invalid body compression".into()),
    };

    if d.u8()? != b'R' {
        return Err("invalid unknown byte".into());
    }

    let class_id = d.u32()?;

    if class_id != T::class_id() {
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

    let mut node_state = NodeState::new(num_nodes as usize);

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

            node_state.set_ref(node_index as usize, file_path);

            Ok(())
        })?;
    }

    if is_body_compressed {
        let body_size = d.u32()?;
        let compressed_body_size = d.u32()?;

        match body_options {
            BodyOptions::Read { .. } => {
                let compressed_body = d.bytes(compressed_body_size as usize)?;

                let mut buf = vec![0; body_size as usize];

                let body = lzo1x_1::decompress_to_slice(&compressed_body, &mut buf).unwrap();

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
    } else {
        match body_options {
            BodyOptions::Read { .. } => {
                let reader = d.into_inner();

                let mut d = Deserializer::new(reader, IdState::new(), node_state);

                T::read_body(&mut node, &mut d)?;

                d.eof()?;
            }
            BodyOptions::Skip => {}
        }
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

    let mut id_state = IdState::new();

    let mut header_chunk_entries = T::header_chunks();

    for (chunk_id, chunk_size) in header_chunks {
        let is_heavy_chunk = chunk_size & 0x80000000 != 0;
        let chunk_size = chunk_size & 0x7FFFFFFF;

        if is_heavy_chunk && !read_heavy_chunks {
            d.skip(chunk_size)?;
        } else {
            let mut d = d.take_with(chunk_size as u64, &mut id_state, ());

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

pub fn read_body_chunks<T: BodyChunks, R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
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
    fn(n: &mut T, d: &mut Deserializer<Take<&mut R>, &mut IdState, ()>) -> Result<()>;

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
    fn(n: &mut T, d: &mut Deserializer<Take<&mut R>, &mut I, &mut N>) -> Result<()>;

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
    fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>>
    where
        Self: Sized;
}

pub trait ReadBody {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
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
