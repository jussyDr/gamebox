use std::ops::Deref;

use crate::Class;

use super::road_chunk::RoadChunk;

/// A placement patch.
#[derive(Default)]
pub struct PlacementPatch {
    parent: RoadChunk,
}

impl Class for PlacementPatch {
    const CLASS_ID: u32 = 0x09160000;
}

impl Deref for PlacementPatch {
    type Target = RoadChunk;

    fn deref(&self) -> &RoadChunk {
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

    use super::PlacementPatch;

    impl ReadBody for PlacementPatch {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }
    impl BodyChunks for PlacementPatch {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }
}
