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
pub struct Key;

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
                let _time = r.f32()?;
                let _exposure = r.f32()?;
                let _max_hdr = r.f32()?;
                let _light_trail_scale = r.f32()?;
                r.u32()?;

                Ok(Key)
            })?;

            Ok(())
        }
    }
}
