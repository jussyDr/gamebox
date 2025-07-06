//! Index buffer.

use crate::ClassId;

/// An index buffer.
#[derive(Default)]
pub struct IndexBuffer {
    indices: Vec<u32>,
}

impl ClassId for IndexBuffer {
    const CLASS_ID: u32 = 0x09057000;
}

impl IndexBuffer {
    /// Indices.
    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }
}

mod read {
    use crate::{
        class::plug::index_buffer::IndexBuffer,
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for IndexBuffer {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for IndexBuffer {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(1, Self::read_chunk_1)]
        }
    }

    impl IndexBuffer {
        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _flags = r.u32()?;
            let offsets: Vec<i16> = r.list_zerocopy()?;

            self.indices = Vec::with_capacity(offsets.len());
            let mut last_index = 0u32;

            for offset in offsets {
                last_index = last_index.checked_add_signed(offset as i32).unwrap();
                self.indices.push(last_index);
            }

            Ok(())
        }
    }
}
