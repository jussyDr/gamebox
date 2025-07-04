//! Placement patch.

use std::ops::{Deref, DerefMut};

use crate::{ClassId, class::plug::road_chunk::RoadChunk};

/// A placement patch.
#[derive(Default)]
pub struct PlacementPatch {
    parent: RoadChunk,
}

impl ClassId for PlacementPatch {
    const CLASS_ID: u32 = 0x09160000;
}

impl Deref for PlacementPatch {
    type Target = RoadChunk;

    fn deref(&self) -> &RoadChunk {
        &self.parent
    }
}

impl DerefMut for PlacementPatch {
    fn deref_mut(&mut self) -> &mut RoadChunk {
        &mut self.parent
    }
}

mod read {
    use crate::{
        class::plug::placement_patch::PlacementPatch,
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for PlacementPatch {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for PlacementPatch {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            []
        }
    }
}
