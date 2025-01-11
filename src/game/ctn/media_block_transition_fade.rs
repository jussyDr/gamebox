//! Media block transition fade.

use bytemuck::cast;
use ordered_float::OrderedFloat;

use crate::{Class, OrderedRgbFloat, RgbFloat};

/// A media block transition fade.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockTransitionFade {
    keys: Vec<Key>,
    color: OrderedRgbFloat,
}

impl Class for MediaBlockTransitionFade {
    const CLASS_ID: u32 = 0x030ab000;
}

impl MediaBlockTransitionFade {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }

    /// Color.
    pub fn color(&self) -> RgbFloat {
        cast(self.color)
    }
}

/// Fading transition media block key.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Key {
    time: OrderedFloat<f32>,
    opacity: OrderedFloat<f32>,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time.0
    }

    /// Opacity.
    pub const fn opacity(&self) -> f32 {
        self.opacity.0
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

    use super::{Key, MediaBlockTransitionFade};

    impl ReadBody for MediaBlockTransitionFade {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), crate::read::Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockTransitionFade {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl MediaBlockTransitionFade {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.keys = r.list(|r| {
                let time = r.f32()?;
                let opacity = r.f32()?;

                Ok(Key {
                    time: OrderedFloat(time),
                    opacity: OrderedFloat(opacity),
                })
            })?;
            self.color = r.rgb_float_ordered()?;
            r.f32()?;

            Ok(())
        }
    }
}
