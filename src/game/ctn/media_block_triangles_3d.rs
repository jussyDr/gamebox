//! Media block triangles 3D.

use std::ops::Deref;

use crate::Class;

use super::media_block_triangles::MediaBlockTriangles;

/// A media block triangles 3D.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockTriangles3D {
    parent: MediaBlockTriangles,
}

impl Class for MediaBlockTriangles3D {
    const CLASS_ID: u32 = 0x0304c000;
}

impl Deref for MediaBlockTriangles3D {
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

    use super::MediaBlockTriangles3D;

    impl ReadBody for MediaBlockTriangles3D {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockTriangles3D {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }
}
