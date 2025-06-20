use crate::Class;

#[derive(Default)]
pub struct IndexBuffer;

impl Class for IndexBuffer {
    fn class_id(&self) -> u32 {
        0x09057000
    }
}

mod read {
    use std::io::Read;

    use crate::{
        class::index_buffer::IndexBuffer,
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks,
            reader::{IdsMut, NodesMut, Reader},
        },
    };

    impl ReadBody for IndexBuffer {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdsMut, impl NodesMut>,
        ) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for IndexBuffer {
        type Parent = Self;

        fn parent(&mut self) -> Option<&mut Self::Parent> {
            None
        }

        fn body_chunks<R: Read, I: IdsMut, N: NodesMut>()
        -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk {
                id: 0x09057001,
                read_fn: Self::read_chunk_1,
                skippable: false,
            }]
            .into_iter()
        }
    }

    impl IndexBuffer {
        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let flags = r.u32()?;
            let num_indices = r.u32()?;
            r.repeat(num_indices as usize, |r| r.i16())?;

            Ok(())
        }
    }
}
