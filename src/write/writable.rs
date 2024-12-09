use std::io::{Error, Seek, Write};

use crate::{Class, END_OF_NODE_MARKER, SKIPPABLE_CHUNK_MARKER};

use super::writer::Writer;

pub trait Sealed: Class + HeaderChunks + BodyChunks {}

pub trait HeaderChunks: Sized {
    fn header_chunks<W, I, N>() -> impl Iterator<Item = HeaderChunk<Self, W, I, N>>;
}

pub struct HeaderChunk<T, W, I, N> {
    pub num: u16,
    pub write_fn: HeaderChunkWriteFn<T, W, I, N>,
    pub heavy: bool,
}

type HeaderChunkWriteFn<T, W, I, N> = fn(&T, &mut Writer<W, I, N>) -> Result<(), Error>;

pub trait BodyChunks: Sized {
    fn body_chunks<W, I, N>() -> impl Iterator<Item = BodyChunk<Self, W, I, N>>;
}

pub struct BodyChunk<T, W, I, N> {
    pub num: u16,
    pub write_fn: BodyChunkWriteFn<T, W, I, N>,
}

pub enum BodyChunkWriteFn<T, W, I, N> {
    Normal(BodyChunkWriteFnNormal<T, W, I, N>),
    Skippable(BodyChunkWriteFnSkippable<T, I, N>),
}

type BodyChunkWriteFnNormal<T, W, I, N> = fn(&T, &mut Writer<W, I, N>) -> Result<(), Error>;

type BodyChunkWriteFnSkippable<T, I, N> =
    fn(&T, &mut Writer<Vec<u8>, &mut I, &mut N>) -> Result<(), Error>;

pub fn write_body<T: Class + BodyChunks, I, N>(
    w: &mut Writer<impl Write + Seek, I, N>,
    node: &T,
) -> Result<(), Error> {
    for body_chunk in T::body_chunks() {
        w.u32(T::CLASS_ID | body_chunk.num as u32)?;

        match body_chunk.write_fn {
            BodyChunkWriteFn::Normal(write_fn) => {
                write_fn(node, w)?;
            }
            BodyChunkWriteFn::Skippable(write_fn) => {
                w.u32(SKIPPABLE_CHUNK_MARKER)?;
                w.byte_buf_inline(|w| write_fn(node, w))?;
            }
        }
    }

    w.u32(END_OF_NODE_MARKER)?;

    Ok(())
}
