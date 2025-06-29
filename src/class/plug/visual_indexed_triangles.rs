use std::ops::{Deref, DerefMut};

use crate::{ClassId, class::plug::visual_indexed::VisualIndexed};

/// A visual indexed triangles.
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
    use std::io::Read;

    use crate::{
        class::plug::visual_indexed_triangles::VisualIndexedTriangles,
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl ReadBody for VisualIndexedTriangles {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for VisualIndexedTriangles {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
            []
        }
    }
}
