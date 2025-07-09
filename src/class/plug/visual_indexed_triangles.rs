//! Visual indexed triangles.

use std::ops::{Deref, DerefMut};

use crate::{ClassId, class::plug::visual_indexed::VisualIndexed};

/// A visual which consists of indexed triangles.
#[derive(Default)]
pub struct VisualIndexedTriangles {
    parent: VisualIndexed,
}

impl ClassId for VisualIndexedTriangles {
    const CLASS_ID: u32 = 0x0901e000;
}

impl Deref for VisualIndexedTriangles {
    type Target = VisualIndexed;

    fn deref(&self) -> &VisualIndexed {
        &self.parent
    }
}

impl DerefMut for VisualIndexedTriangles {
    fn deref_mut(&mut self) -> &mut VisualIndexed {
        &mut self.parent
    }
}

mod read {
    use crate::{
        class::plug::visual_indexed_triangles::VisualIndexedTriangles,
        read::{BodyChunk, BodyChunks, BodyReader, Error, ReadBody, read_body_chunks},
    };

    impl ReadBody for VisualIndexedTriangles {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for VisualIndexedTriangles {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            []
        }
    }
}
