//! Media block camera effect shake.

use ordered_float::OrderedFloat;

use crate::Class;

/// Camera effect shake media block.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockCameraEffectShake {
    keys: Vec<Key>,
}

impl Class for MediaBlockCameraEffectShake {
    const CLASS_ID: u32 = 0x030a4000;
}

/// Camera effect shake media block key.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Key {
    time: OrderedFloat<f32>,
    intensity: OrderedFloat<f32>,
    speed: OrderedFloat<f32>,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time.0
    }

    /// Intensity.
    pub const fn intensity(&self) -> f32 {
        self.intensity.0
    }

    /// Speed.
    pub const fn speed(&self) -> f32 {
        self.speed.0
    }
}

mod read {
    use std::io::{Read, Seek};

    use ordered_float::OrderedFloat;

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Key, MediaBlockCameraEffectShake};

    impl ReadBody for MediaBlockCameraEffectShake {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockCameraEffectShake {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl MediaBlockCameraEffectShake {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.keys = r.list(|r| {
                let time = r.f32()?;
                let intensity = r.f32()?;
                let speed = r.f32()?;

                Ok(Key {
                    time: OrderedFloat(time),
                    intensity: OrderedFloat(intensity),
                    speed: OrderedFloat(speed),
                })
            })?;

            Ok(())
        }
    }
}
