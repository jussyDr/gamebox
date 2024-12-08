//! Media block Fx colors.

use crate::Class;

/// Fx colors media block.
#[derive(Default)]
pub struct MediaBlockFxColors {
    keys: Vec<Key>,
}

impl Class for MediaBlockFxColors {
    const CLASS_ID: u32 = 0x03080000;
}

impl MediaBlockFxColors {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Fx colors media block key.
pub struct Key;

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Key, MediaBlockFxColors};

    impl ReadBody for MediaBlockFxColors {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockFxColors {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(3, Self::read_chunk_3)].into_iter()
        }
    }

    impl MediaBlockFxColors {
        fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.keys = r.list(|r| {
                let _time = r.f32()?;
                let _intensity = r.f32()?;
                let _blend_z = r.f32()?;
                let _distance = r.f32()?;
                let _far_distance = r.f32()?;
                let _inverse = r.f32()?;
                let _hue = r.f32()?;
                let _saturation = r.f32()?;
                let _brightness = r.f32()?;
                let _contrast = r.f32()?;
                let _rgb = r.vec3::<f32>()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                let _far_inverse = r.f32()?;
                let _far_hue = r.f32()?;
                let _far_saturation = r.f32()?;
                let _far_brightness = r.f32()?;
                let _far_contrast = r.f32()?;
                let _far_rgb = r.vec3::<f32>()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;

                Ok(Key)
            })?;

            Ok(())
        }
    }
}
