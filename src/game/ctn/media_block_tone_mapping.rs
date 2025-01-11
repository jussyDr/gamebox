//! Tone mapping.

use ordered_float::OrderedFloat;

use crate::Class;

/// Tone mapping.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
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
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Key {
    time: OrderedFloat<f32>,
    exposure: OrderedFloat<f32>,
    max_hdr: OrderedFloat<f32>,
    light_trail_scale: OrderedFloat<f32>,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time.0
    }

    /// Exposure.
    pub const fn exposure(&self) -> f32 {
        self.exposure.0
    }

    /// Max HDR.
    pub const fn max_hdr(&self) -> f32 {
        self.max_hdr.0
    }

    /// Light trail scale.
    pub const fn light_trail_scale(&self) -> f32 {
        self.light_trail_scale.0
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
                    time: OrderedFloat(time),
                    exposure: OrderedFloat(exposure),
                    max_hdr: OrderedFloat(max_hdr),
                    light_trail_scale: OrderedFloat(light_trail_scale),
                })
            })?;

            Ok(())
        }
    }
}
