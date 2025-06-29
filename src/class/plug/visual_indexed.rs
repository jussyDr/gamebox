use std::ops::{Deref, DerefMut};

use crate::{
    Class,
    class::plug::{index_buffer::IndexBuffer, visual_3d::Visual3D},
};

/// A visual indexed.
#[derive(Default)]
pub struct VisualIndexed {
    parent: Visual3D,
    index_buffer: IndexBuffer,
}

impl VisualIndexed {
    pub fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
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

impl DerefMut for VisualIndexed {
    fn deref_mut(&mut self) -> &mut Visual3D {
        &mut self.parent
    }
}

mod read {
    use std::io::Read;

    use crate::{
        class::plug::visual_indexed::VisualIndexed,
        read::{
            BodyChunk, BodyChunks, Error, read_node,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl BodyChunks for VisualIndexed {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(1, Self::read_chunk_1)]
        }
    }

    impl VisualIndexed {
        fn read_chunk_1(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            if r.bool32()? {
                self.index_buffer = read_node(r)?;
            }

            Ok(())
        }
    }
}
