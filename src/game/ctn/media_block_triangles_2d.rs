//! Media block triangles 2D.

use std::ops::Deref;

use crate::Class;

use super::media_block_triangles::MediaBlockTriangles;

/// 2D triangles media block.
#[derive(PartialEq, Eq, Hash, Default)]
pub struct MediaBlockTriangles2D {
    parent: MediaBlockTriangles,
}

impl Class for MediaBlockTriangles2D {
    const CLASS_ID: u32 = 0x0304b000;
}

impl Deref for MediaBlockTriangles2D {
    type Target = MediaBlockTriangles;

    fn deref(&self) -> &MediaBlockTriangles {
        &self.parent
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::MediaBlockTriangles2D;

    impl ReadBody for MediaBlockTriangles2D {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockTriangles2D {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }
}
