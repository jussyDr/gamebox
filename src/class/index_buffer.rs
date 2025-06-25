use crate::Class;

#[derive(Default)]
pub struct IndexBuffer {
    indices: Vec<u32>,
}

impl Class for IndexBuffer {
    const CLASS_ID: u32 = 0x09057000;
}

impl IndexBuffer {
    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }
}

mod read {
    use std::io::Read;

    use crate::{
        class::index_buffer::IndexBuffer,
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl ReadBody for IndexBuffer {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for IndexBuffer {
        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(0x09057001, Self::read_chunk_1)]
        }
    }

    impl IndexBuffer {
        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let flags = r.u32()?;
            let mut last_index = 0u32;
            self.indices = r.list(|r| {
                let offset = r.i16()?;
                last_index = last_index.checked_add_signed(offset as i32).unwrap();

                Ok(last_index)
            })?;

            Ok(())
        }
    }
}
