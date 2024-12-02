use std::io::{Error, Seek, Write};

use crate::{Class, END_OF_NODE_MARKER, SKIPPABLE_CHUNK_MARKER};

use super::writer::Writer;

pub trait Sealed: Class + BodyChunks {}

pub trait BodyChunks: Sized {
    fn body_chunks<W, I, N>() -> impl Iterator<Item = BodyChunk<Self, W, I, N>>;
}

pub struct BodyChunk<T, W, I, N> {
    num: u16,
    write_fn: BodyChunkWriteFn<T, W, I, N>,
}

enum BodyChunkWriteFn<T, W, I, N> {
    Normal(BodyChunkWriteFnNormal<T, W, I, N>),
    Skippable(BodyChunkWriteFnSkippable<T, W, I, N>),
}

type BodyChunkWriteFnNormal<T, W, I, N> = fn(&T, &mut Writer<W, I, N>) -> Result<(), Error>;

type BodyChunkWriteFnSkippable<T, W, I, N> = fn(&T, &mut Writer<W, I, N>) -> Result<(), Error>;

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
                w.byte_buf(|w| write_fn(node, w))?;
            }
        }
    }

    w.u32(END_OF_NODE_MARKER)?;

    Ok(())
}
