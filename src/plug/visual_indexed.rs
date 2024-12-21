//! Visual indexed.

use std::ops::Deref;

use crate::Class;

use super::{index_buffer::IndexBuffer, visual3d::Visual3D};

/// A visual indexed.
#[derive(Default, Debug)]
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
    /// Index buffer of the visual.
    pub const fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::index_buffer::IndexBuffer,
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

        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(1, Self::read_chunk_1)].into_iter()
        }
    }

    impl VisualIndexed {
        fn read_chunk_1(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            if r.bool()? {
                self.index_buffer = IndexBuffer::read_from_body(r)?;
            }

            Ok(())
        }
    }
}
