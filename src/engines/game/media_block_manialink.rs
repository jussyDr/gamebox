use std::io::Read;

use crate::{
    read::{
        readable::{BodyChunk, BodyChunks},
        IdStateMut, Reader,
    },
    Error,
};

/// A manialink media block.
#[derive(Default)]
pub struct MediaBlockManialink;

impl BodyChunks for MediaBlockManialink {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(1, |n, r| Self::read_chunk_1(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaBlockManialink {
    fn read_chunk_1<N>(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, N>,
    ) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 0 {
            return Err(Error);
        }

        let _start = r.f32()?;
        let _end = r.f32()?;
        let _manialink_url = r.string()?;

        Ok(())
    }
}
