use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunksInline},
    Error, Reader,
};

/// TODO.
#[derive(Default)]
pub struct WaypointSpecialProperty;

impl BodyChunksInline for WaypointSpecialProperty {
    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 2] = [
            (0, |n, r| Self::read_chunk_0(n, r), false),
            (1, |n, r| Self::read_chunk_1(n, r), true),
        ];

        chunks.into_iter()
    }
}

impl WaypointSpecialProperty {
    fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 2 {
            return Err(Error);
        }

        let _tag = r.string()?;
        let _order = r.u32()?;

        Ok(())
    }

    fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;
        r.u32()?;

        Ok(())
    }
}
