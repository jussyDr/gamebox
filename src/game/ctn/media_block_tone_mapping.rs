//! Tone mapping.

use crate::Class;

/// Tone mapping.
#[derive(Default)]
pub struct MediaBlockToneMapping {
    keys: Vec<Key>,
}

impl Class for MediaBlockToneMapping {
    const CLASS_ID: u32 = 0x03127000;
}

impl MediaBlockToneMapping {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Tone mapping media block key.
pub struct Key {
    time: f32,
    exposure: f32,
    max_hdr: f32,
    light_trail_scale: f32,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time
    }

    /// Exposure.
    pub const fn exposure(&self) -> f32 {
        self.exposure
    }

    /// Max HDR.
    pub const fn max_hdr(&self) -> f32 {
        self.max_hdr
    }

    /// Light trail scale.
    pub const fn light_trail_scale(&self) -> f32 {
        self.light_trail_scale
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Key, MediaBlockToneMapping};

    impl ReadBody for MediaBlockToneMapping {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockToneMapping {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(4, Self::read_chunk_4)].into_iter()
        }
    }

    impl MediaBlockToneMapping {
        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.keys = r.list(|r| {
                let time = r.f32()?;
                let exposure = r.f32()?;
                let max_hdr = r.f32()?;
                let light_trail_scale = r.f32()?;
                r.u32()?;

                Ok(Key {
                    time,
                    exposure,
                    max_hdr,
                    light_trail_scale,
                })
            })?;

            Ok(())
        }
    }
}
