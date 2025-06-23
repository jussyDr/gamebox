use crate::{Class, class::visual_indexed::VisualIndexed};

#[derive(Default)]
pub struct VisualIndexedTriangles {
    parent: VisualIndexed,
}

impl Class for VisualIndexedTriangles {
    const CLASS_ID: u32 = 0x0901e000;
}

mod read {
    use std::io::Read;

    use crate::{
        class::{visual_indexed::VisualIndexed, visual_indexed_triangles::VisualIndexedTriangles},
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
        type Parent = VisualIndexed;

        fn parent(&mut self) -> Option<&mut Self::Parent> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
            []
        }
    }
}
