//! Media block transition fade.

use crate::Class;

/// A media block transition fade.
#[derive(Default)]
pub struct MediaBlockTransitionFade {
    keys: Vec<Key>,
}

impl Class for MediaBlockTransitionFade {
    const CLASS_ID: u32 = 0x030ab000;
}

impl MediaBlockTransitionFade {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Fading transition media block key.
pub struct Key;

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
                let _time = r.f32()?;
                let _opacity = r.f32()?;

                Ok(Key)
            })?;
            let _color = r.vec3::<f32>()?;
            r.f32()?;

            Ok(())
        }
    }
}
