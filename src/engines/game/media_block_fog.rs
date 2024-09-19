use std::io::Read;

use crate::{
    read::{
        readable::{BodyChunk, BodyChunks},
        Reader,
    },
    Error,
};

/// A fog media block.
pub struct MediaBlockFog;

impl BodyChunks for MediaBlockFog {
    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(0, |n, r| Self::read_chunk_0(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaBlockFog {
    fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 2 {
            return Err(Error);
        }

        let _keys = r.list(|r| {
            let _time = r.f32()?;
            let _intensity = r.f32()?;
            let _sky_intensity = r.f32()?;
            let _distance = r.f32()?;
            let _coefficient = r.f32()?;
            let _color = r.vec3::<f32>()?;
            let _clouds_opacity = r.f32()?;
            let _clouds_speed = r.f32()?;

            Ok(())
        })?;

        Ok(())
    }
}
