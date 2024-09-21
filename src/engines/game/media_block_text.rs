use std::io::Read;

use crate::{
    engines::control::EffectSimi,
    read::{
        readable::{BodyChunk, BodyChunks},
        Error, IdStateMut, NodeStateMut, Reader,
    },
};

/// A text media block.
pub struct MediaBlockText;

impl BodyChunks for MediaBlockText {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 2] = [
            (1, |n, r| Self::read_chunk_1(n, r), false),
            (2, |n, r| Self::read_chunk_2(n, r), false),
        ];

        chunks.into_iter()
    }
}

impl MediaBlockText {
    fn read_chunk_1(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<(), Error> {
        let _text = r.string()?;
        let _effect = r.node::<EffectSimi>()?;

        Ok(())
    }

    fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _color = r.vec3::<f32>()?;

        Ok(())
    }
}
