use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunks},
    Error, Reader,
};

/// A tone mapping media block.
pub struct MediaBlockToneMapping;

impl BodyChunks for MediaBlockToneMapping {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(4, |n, r| Self::read_chunk_4(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaBlockToneMapping {
    fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _keys = r.list(|r| {
            let _time = r.f32()?;
            let _exposure = r.f32()?;
            let _max_hdr = r.f32()?;
            let _light_trail_scale = r.f32()?;
            r.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}
