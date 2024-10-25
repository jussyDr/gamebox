use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunks},
    Error, Reader,
};

/// A ent record data.
#[derive(Default)]
pub struct EntRecordData;

impl BodyChunks for EntRecordData {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(0, |n, r| Self::read_chunk_0(n, r), false)];

        chunks.into_iter()
    }
}

impl EntRecordData {
    fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 10 {
            return Err(Error);
        }

        let _uncompressed_size = r.u32()?;
        let _data = r.byte_buf()?;

        Ok(())
    }
}
