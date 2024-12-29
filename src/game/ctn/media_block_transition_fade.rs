//! Media block transition fade.

use crate::{Class, RgbFloat};

/// A media block transition fade.
#[derive(Default)]
pub struct MediaBlockTransitionFade {
    keys: Vec<Key>,
    color: RgbFloat,
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
    pub const fn color(&self) -> RgbFloat {
        self.color
    }
}

/// Fading transition media block key.
pub struct Key {
    time: f32,
    opacity: f32,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time
    }

    /// Opacity.
    pub const fn opacity(&self) -> f32 {
        self.opacity
    }
}

mod read {
    use std::io::{Read, Seek};

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

                Ok(Key { time, opacity })
            })?;
            self.color = r.rgb_float()?;
            r.f32()?;

            Ok(())
        }
    }
}
