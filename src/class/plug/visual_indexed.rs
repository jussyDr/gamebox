//! Visual indexed.

use std::ops::{Deref, DerefMut};

use crate::{
    ClassId,
    class::plug::{index_buffer::IndexBuffer, visual_3d::Visual3D},
};

/// A visual indexed.
#[derive(Default)]
pub struct VisualIndexed {
    parent: Visual3D,
    index_buffer: IndexBuffer,
}

impl VisualIndexed {
    /// Index buffer.
    pub fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
}

impl ClassId for VisualIndexed {
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

    use crate::{
        class::plug::visual_indexed::VisualIndexed,
        read::{BodyChunk, BodyChunks, BodyReader, Error, read_node_from_body},
    };

    impl BodyChunks for VisualIndexed {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(1, Self::read_chunk_1)]
        }
    }

    impl VisualIndexed {
        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            if r.bool32()? {
                self.index_buffer = read_node_from_body(r)?;
            }

            Ok(())
        }
    }
}
