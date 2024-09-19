use std::io::Read;

use crate::{
    read::{
        readable::{BodyChunk, BodyChunks},
        IdStateMut, NodeStateMut, Reader,
    },
    Error,
};

use super::MediaBlock;

/// A media track.
#[derive(Default)]
pub struct MediaTrack;

impl BodyChunks for MediaTrack {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 2] = [
            (1, |n, r| Self::read_chunk_1(n, r), false),
            (5, |n, r| Self::read_chunk_5(n, r), false),
        ];

        chunks.into_iter()
    }
}

impl MediaTrack {
    fn read_chunk_1(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<(), Error> {
        let _name = r.string()?;
        let list_version = r.u32()?;

        if list_version != 10 {
            return Err(Error);
        }

        let _blocks = r.list(|r| MediaBlock::read(r))?;
        r.u32()?;

        Ok(())
    }

    fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 1 {
            return Err(Error);
        }

        let _is_keep_playing = r.bool()?;
        let _is_read_only = r.bool()?;
        let _is_cycling = r.bool()?;
        r.f32()?;
        r.f32()?;

        Ok(())
    }
}
