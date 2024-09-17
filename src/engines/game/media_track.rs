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
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(1, |n, r| Self::read_chunk_1(n, r), false)];

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

        let _blocks = r.list(|r| r.node_abstract::<MediaBlock>())?;
        r.u32()?;

        Ok(())
    }
}
