use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunks},
    Error, Reader,
};

/// A FX colors block.
pub struct MediaBlockFxColors;

impl BodyChunks for MediaBlockFxColors {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(3, |n, r| Self::read_chunk_3(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaBlockFxColors {
    fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _keys = r.list(|r| {
            let _time = r.f32()?;
            let _intensity = r.f32()?;
            let _blend_z = r.f32()?;
            let _distance = r.f32()?;
            let _far_distance = r.f32()?;
            let _inverse = r.f32()?;
            let _hue = r.f32()?;
            let _saturation = r.f32()?;
            let _brightness = r.f32()?;
            let _constrast = r.f32()?;
            let _rgb = r.vec3::<f32>()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            let _far_inverse = r.f32()?;
            let _far_hue = r.f32()?;
            let _far_saturation = r.f32()?;
            let _far_brightness = r.f32()?;
            let _far_constrast = r.f32()?;
            let _far_rgb = r.vec3::<f32>()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;

            Ok(())
        })?;

        Ok(())
    }
}
