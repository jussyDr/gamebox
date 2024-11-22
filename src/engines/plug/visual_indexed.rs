use std::ops::Deref;

use crate::Class;

use super::{index_buffer::IndexBuffer, visual3d::Visual3D};

#[derive(Default)]
pub struct VisualIndexed {
    parent: Visual3D,
    index_buffer: IndexBuffer,
}

impl Class for VisualIndexed {
    const CLASS_ID: u32 = 0x0906a000;
}

impl Deref for VisualIndexed {
    type Target = Visual3D;

    fn deref(&self) -> &Visual3D {
        &self.parent
    }
}

impl VisualIndexed {
    pub const fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
}

mod read {
    use std::io::Read;

    use crate::{
        engines::plug::index_buffer::IndexBuffer,
        read::{
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::VisualIndexed;

    impl BodyChunks for VisualIndexed {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(1, Self::read_chunk_1)].into_iter()
        }
    }

    impl VisualIndexed {
        fn read_chunk_1(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            if r.bool()? {
                let mut index_buffer = IndexBuffer::default();
                index_buffer.read_body(r)?;
                self.index_buffer = index_buffer;
            }

            Ok(())
        }
    }
}
