//! Media block fog.

use crate::Class;

/// A media block fog.
#[derive(Default)]
pub struct MediaBlockFog {
    keys: Vec<Key>,
}

impl Class for MediaBlockFog {
    const CLASS_ID: u32 = 0x03199000;
}

impl MediaBlockFog {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Fog media block key.
pub struct Key;

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Key, MediaBlockFog};

    impl ReadBody for MediaBlockFog {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockFog {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl MediaBlockFog {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            self.keys = r.list(|r| {
                let _time = r.f32()?;
                let _intensity = r.f32()?;
                let _sky_intensity = r.f32()?;
                let _distance = r.f32()?;
                let _coefficient = r.f32()?;
                let _color = r.vec3::<f32>()?;
                let _clouds_opacity = r.f32()?;
                let _clouds_speed = r.f32()?;

                Ok(Key)
            })?;

            Ok(())
        }
    }
}
