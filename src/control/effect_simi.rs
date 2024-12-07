//! Effect simi.

use crate::Class;

/// Effect simi.
#[derive(Default)]
pub struct EffectSimi {
    keys: Vec<Key>,
}

impl Class for EffectSimi {
    const CLASS_ID: u32 = 0x07010000;
}

impl EffectSimi {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Effect simi key.
pub struct Key;

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{EffectSimi, Key};

    impl ReadBody for EffectSimi {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for EffectSimi {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(5, Self::read_chunk_5)].into_iter()
        }
    }

    impl EffectSimi {
        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.keys = r.list(|r| {
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

                Ok(Key)
            })?;
            let _centered = r.bool()?;
            let _color_blend_mode = r.u32()?;
            let _is_continuous_effect = r.bool()?;
            let _is_interpolated = r.bool()?;

            Ok(())
        }
    }
}
