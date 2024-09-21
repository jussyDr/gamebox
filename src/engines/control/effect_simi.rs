use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunks},
    Error, Reader,
};

enum ColorBlendMode {
    Set,
    Mult,
}

impl ColorBlendMode {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let color_blend_mode = match r.u32()? {
            0 => Self::Set,
            1 => Self::Mult,
            _ => return Err(Error),
        };

        Ok(color_blend_mode)
    }
}

/// An effect simi.
#[derive(Default)]
pub struct EffectSimi;

impl BodyChunks for EffectSimi {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(5, |n, r| Self::read_chunk_5(n, r), false)];

        chunks.into_iter()
    }
}

impl EffectSimi {
    fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _keys = r.list(|r| {
            let _time = r.f32()?;
            let _position = r.vec2::<f32>()?;
            let _rotation = r.f32()?;
            let _scale = r.vec2::<f32>()?;
            let _opacity = r.f32()?;
            let _depth = r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;

            Ok(())
        })?;
        let _centered = r.bool()?;
        let _color_blend_mode = ColorBlendMode::read(r)?;
        let _is_continuous_effect = r.bool()?;
        let _is_interpolated = r.bool()?;

        Ok(())
    }
}
