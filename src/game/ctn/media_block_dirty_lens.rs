//! Media block dirty lens.

use ordered_float::OrderedFloat;

use crate::Class;

/// Dirty lens media block.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockDirtyLens {
    keys: Vec<Key>,
}

impl Class for MediaBlockDirtyLens {
    const CLASS_ID: u32 = 0x03165000;
}

impl MediaBlockDirtyLens {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Dirty lens media block key.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Key {
    time: OrderedFloat<f32>,
    intensity: OrderedFloat<f32>,
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
}

mod read {
    use std::io::{Read, Seek};

    use ordered_float::OrderedFloat;

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Key, MediaBlockDirtyLens};

    impl ReadBody for MediaBlockDirtyLens {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockDirtyLens {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl MediaBlockDirtyLens {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            self.keys = r.list(|r| {
                let time = r.f32()?;
                let intensity = r.f32()?;

                Ok(Key {
                    time: OrderedFloat(time),
                    intensity: OrderedFloat(intensity),
                })
            })?;

            Ok(())
        }
    }
}

mod write {
    use std::io::Write;

    use crate::write::{
        writable::{write_body_chunks, WriteBody},
        writer::{IdStateMut, NodeStateMut},
        BodyChunk, BodyChunks, Error, Writer,
    };

    use super::MediaBlockDirtyLens;

    impl WriteBody for MediaBlockDirtyLens {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for MediaBlockDirtyLens {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
