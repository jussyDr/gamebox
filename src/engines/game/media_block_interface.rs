use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunks},
    Error, Reader,
};

/// An interface media block.
pub struct MediaBlockInterface;

impl BodyChunks for MediaBlockInterface {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(0, |n, r| Self::read_chunk_0(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaBlockInterface {
    fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 0 {
            return Err(Error);
        }

        let _start = r.f32()?;
        let _end = r.f32()?;
        let _show_interface = r.bool()?;
        let _mania_link = r.string()?;

        Ok(())
    }
}
