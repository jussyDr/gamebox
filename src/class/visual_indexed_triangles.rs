use crate::{Class, class::visual_indexed::VisualIndexed};

#[derive(Default)]
pub struct VisualIndexedTriangles {
    parent: VisualIndexed,
}

impl Class for VisualIndexedTriangles {
    fn class_id(&self) -> u32 {
        0x0901e000
    }
}

mod read {
    use std::io::Read;

    use crate::{
        class::{visual_indexed::VisualIndexed, visual_indexed_triangles::VisualIndexedTriangles},
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks,
            reader::{IdsMut, NodesMut, Reader},
        },
    };

    impl ReadBody for VisualIndexedTriangles {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdsMut, impl NodesMut>,
        ) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for VisualIndexedTriangles {
        type Parent = VisualIndexed;

        fn parent(&mut self) -> Option<&mut Self::Parent> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I: IdsMut, N: NodesMut>()
        -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }
}
