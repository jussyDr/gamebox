use std::{
    io::{BufRead, Cursor, Read, Seek},
    rc::Rc,
};

use serde_jsonrc::Value;

use crate::{
    common::{Class, END_OF_NODE_MARKER, HEAVY_CHUNK_MARKER_BIT, SKIPPABLE_CHUNK_MARKER},
    deserialize::{Deserializer, IdState, NodeRef, NodeState, Take},
};

use super::{file::GbxFile, BodyOptions, HeaderOptions, Result};

pub fn read_gbx<
    T: Default + Class + HeaderChunks + for<'a> ReadBody<Cursor<&'a [u8]>, IdState, NodeState>,
>(
    reader: impl Read,
    header_options: HeaderOptions,
    body_options: BodyOptions,
) -> Result<T> {
    let mut node = T::default();

    let assume_no_header_data = matches!(
        header_options,
        HeaderOptions::Skip {
            assume_size_zero: true,
        }
    );

    let mut gbx_file = GbxFile::read(reader, assume_no_header_data)?;

    if gbx_file.class_id() != T::CLASS_ID {
        return Err("class id does not match".into());
    }

    if let HeaderOptions::Read { read_heavy_chunks } = header_options {
        read_header(
            &mut node,
            Deserializer::new(Cursor::new(gbx_file.header_data()), (), ()),
            read_heavy_chunks,
        )?;
    }

    let mut node_state = NodeState::new(gbx_file.num_node_refs() as usize);

    for (index, path) in gbx_file.external_node_refs() {
        node_state.set_node_ref(
            *index as usize,
            NodeRef::External {
                path: Rc::from(path.to_owned()),
            },
        )?;
    }

    match body_options {
        BodyOptions::Read { .. } => {
            let mut d = Deserializer::new(
                Cursor::new(gbx_file.body_data()?),
                IdState::new(),
                node_state,
            );

            T::read_body(&mut node, &mut d)?;

            d.eof()?;
        }
        BodyOptions::Skip => {}
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
        let is_heavy_chunk = chunk_size & HEAVY_CHUNK_MARKER_BIT != 0;
        let chunk_size = chunk_size & !HEAVY_CHUNK_MARKER_BIT;

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

pub fn read_body_chunks<R: Read, I, N, T: BodyChunks<R, I, N>>(
    node: &mut T,
    d: &mut Deserializer<R, I, N>,
) -> Result<()> {
    let mut body_chunk_entries = T::body_chunks();

    loop {
        let chunk_id = d.u32()?;

        if chunk_id == END_OF_NODE_MARKER {
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
                if d.u32()? != SKIPPABLE_CHUNK_MARKER {
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
        reader: impl Read,
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

pub trait BodyChunks<R, I, N> {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>>
    where
        Self: Sized;
}

pub trait ReadBody<R, I, N> {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()>;
}

pub trait ReadJson {
    const CLASS_NAME: &'static str;

    fn read(json: Value) -> Result<Self>
    where
        Self: Sized;
}

pub fn read_json<T: ReadJson>(reader: impl Read) -> Result<T> {
    let mut value: Value = serde_jsonrc::from_reader(reader).map_err(|_| "failed to parse JSON")?;
    let object = value.as_object_mut().ok_or("expected an object")?;
    let class_name = object.get("ClassId").ok_or("expected key")?;

    if class_name != T::CLASS_NAME {
        return Err("class name does not match".into());
    }

    object.remove("ClassId");

    T::read(value)
}
