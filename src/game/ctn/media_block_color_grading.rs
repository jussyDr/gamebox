//! Color grading.

use crate::Class;

/// Media block color grading.
#[derive(Default)]
pub struct MediaBlockColorGrading {
    keys: Vec<Key>,
}

impl Class for MediaBlockColorGrading {
    const CLASS_ID: u32 = 0x03186000;
}

impl MediaBlockColorGrading {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Color grading media block key.
pub struct Key {
    time: f32,
    intensity: f32,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time
    }

    /// Intensity.
    pub const fn intensity(&self) -> f32 {
        self.intensity
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Key, MediaBlockColorGrading};

    impl ReadBody for MediaBlockColorGrading {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockColorGrading {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(0, Self::read_chunk_0),
                BodyChunk::normal(1, Self::read_chunk_1),
            ]
            .into_iter()
        }
    }

    impl MediaBlockColorGrading {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _image = r.pack_desc()?;

            Ok(())
        }

        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _keys = r.list(|r| {
                let time = r.f32()?;
                let intensity = r.f32()?;

                Ok(Key { time, intensity })
            })?;

            Ok(())
        }
    }
}
