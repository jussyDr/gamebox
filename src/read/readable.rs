use std::io::{Read, Seek};

use crate::{read::ErrorKind, Class, END_OF_NODE_MARKER, SKIPPABLE_CHUNK_MARKER};

use super::{
    reader::{IdStateMut, NodeStateMut, Reader},
    Error,
};

pub trait Sealed: Class + ReadBody {}

pub trait ReadBody: Send + Sync + Default {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<(), Error>;

    fn read_from_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        r: &mut Reader<R, I, N>,
    ) -> Result<Self, Error> {
        let mut node = Self::default();
        node.read_body(r)?;

        Ok(node)
    }
}

pub fn read_body_chunks<T: Class + BodyChunks>(
    node: &mut T,
    r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
) -> Result<(), Error> {
    let chunk_id = read_body_chunks_inner(node, r)?;

    if chunk_id != END_OF_NODE_MARKER {
        return Err(Error::new(ErrorKind::Unsupported(format!(
            "chunk: {chunk_id:08X?}"
        ))));
    }

    Ok(())
}

fn read_body_chunks_inner<T: Class + BodyChunks>(
    node: &mut T,
    r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
) -> Result<u32, Error> {
    let mut chunk_id = match node.parent() {
        Some(parent) => read_body_chunks_inner(parent, r)?,
        None => r.u32()?,
    };

    let mut chunks = T::body_chunks();

    loop {
        if chunk_id == END_OF_NODE_MARKER {
            break;
        }

        let class_id = chunk_id & 0xfffff000;

        if class_id != T::CLASS_ID {
            break;
        }

        let chunk_num = (chunk_id & 0x00000fff) as u16;

        let chunk = chunks.find(|chunk| chunk.num == chunk_num).ok_or_else(|| {
            Error::new(ErrorKind::Unsupported(format!(
                "unknown chunk: {chunk_num}"
            )))
        })?;

        match chunk.read_fn {
            BodyChunkReadFn::Normal(read_fn) => {
                read_fn(node, r)?;
            }
            BodyChunkReadFn::Skippable(read_fn) => {
                if r.u32()? != SKIPPABLE_CHUNK_MARKER {
                    return Err(Error::new(ErrorKind::Format("expected skippable chunk")));
                }

                let size = r.u32()?;

                read_fn(node, r)?;
            }
        }

        chunk_id = r.u32()?;
    }

    Ok(chunk_id)
}

pub trait BodyChunks: Sized + Class {
    fn parent(&mut self) -> Option<&mut impl BodyChunks> {
        None::<&mut Self>
    }

    fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>>;
}

pub struct BodyChunk<T, R, I, N> {
    num: u16,
    read_fn: BodyChunkReadFn<T, R, I, N>,
}

pub enum BodyChunkReadFn<T, R, I, N> {
    Normal(BodyChunkReadFnNormal<T, R, I, N>),
    Skippable(BodyChunkReadFnSkippable<T, R, I, N>),
}

pub type BodyChunkReadFnNormal<T, R, I, N> = fn(&mut T, &mut Reader<R, I, N>) -> Result<(), Error>;

pub type BodyChunkReadFnSkippable<T, R, I, N> =
    fn(&mut T, &mut Reader<R, I, N>) -> Result<(), Error>;

impl<T, R, I, N> BodyChunk<T, R, I, N> {
    pub const fn normal(num: u16, read_fn: BodyChunkReadFnNormal<T, R, I, N>) -> Self {
        Self {
            num,
            read_fn: BodyChunkReadFn::Normal(read_fn),
        }
    }

    pub const fn skippable(num: u16, read_fn: BodyChunkReadFnSkippable<T, R, I, N>) -> Self {
        Self {
            num,
            read_fn: BodyChunkReadFn::Skippable(read_fn),
        }
    }
}
