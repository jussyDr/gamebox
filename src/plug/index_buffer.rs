//! Index buffer.

use crate::Class;

/// An index buffer.
#[derive(Default)]
pub struct IndexBuffer {
    indices: Vec<u32>,
}

impl Class for IndexBuffer {
    const CLASS_ID: u32 = 0x09057000;
}

impl IndexBuffer {
    pub const fn indices(&self) -> &Vec<u32> {
        &self.indices
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::IndexBuffer;

    impl ReadBody for IndexBuffer {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for IndexBuffer {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(1, Self::read_chunk_1)].into_iter()
        }
    }

    impl IndexBuffer {
        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _flags = r.u32()?;

            let mut last_index = 0;
            self.indices = r.list(|r| {
                last_index += r.i16()? as i32;

                Ok(last_index as u32)
            })?;

            Ok(())
        }
    }
}
