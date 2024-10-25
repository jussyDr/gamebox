use std::io::Read;

use crate::{
    engines::control::EffectSimi,
    read::{
        readable::{BodyChunk, BodyChunks},
        Error, IdStateMut, NodeStateMut, Reader,
    },
};

/// An image media block.
pub struct MediaBlockImage;

impl BodyChunks for MediaBlockImage {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(0, |n, r| Self::read_chunk_0(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaBlockImage {
    fn read_chunk_0(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<(), Error> {
        let _effect = r.node::<EffectSimi>()?;
        let _image = r.pack_desc()?;

        Ok(())
    }
}
