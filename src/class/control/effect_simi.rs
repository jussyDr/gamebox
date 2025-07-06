//! Effect simi.

use crate::ClassId;

/// Effect simi.
#[derive(Default)]
pub struct EffectSimi;

impl ClassId for EffectSimi {
    const CLASS_ID: u32 = 0x07010000;
}

mod read {
    use crate::{
        class::control::effect_simi::EffectSimi,
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for EffectSimi {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for EffectSimi {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(5, Self::read_chunk_5)]
        }
    }

    impl EffectSimi {
        fn read_chunk_5(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _keys = r.list(|r| {
                let _time = r.f32()?;
                let _position = r.vec2()?;
                let _rotation = r.f32()?;
                let _scale = r.vec2()?;
                let _opacity = r.f32()?;
                let _depth = r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;

                Ok(())
            })?;
            let _centered = r.bool32()?;
            let _color_blend_mode = r.u32()?;
            let _is_continuous_effect = r.bool32()?;
            let _is_interpolated = r.bool32()?;

            Ok(())
        }
    }
}
