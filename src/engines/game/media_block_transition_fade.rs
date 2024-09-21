use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunks},
    Error, Reader,
};

/// A transition fade media block.
pub struct MediaBlockTransitionFade;

impl BodyChunks for MediaBlockTransitionFade {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(0, |n, r| Self::read_chunk_0(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaBlockTransitionFade {
    fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _keys = r.list(|r| {
            let _time = r.f32()?;
            let _opacity = r.f32()?;

            Ok(())
        })?;
        let _color = r.vec3::<f32>()?;
        r.f32()?;

        Ok(())
    }
}
