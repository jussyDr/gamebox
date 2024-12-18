//! Media block dirty lens.

use crate::Class;

/// Dirty lens media block.
#[derive(Default)]
pub struct MediaBlockDirtyLens;

impl Class for MediaBlockDirtyLens {
    const CLASS_ID: u32 = 0x03165000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::MediaBlockDirtyLens;

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

            let _keys = r.list(|r| {
                let _time = r.f32()?;
                let _intensity = r.f32()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
