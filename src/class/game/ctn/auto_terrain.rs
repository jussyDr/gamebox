//! Auto terrain.

use std::sync::Arc;

use crate::{ClassId, UVec3, class::game::ctn::zone_genealogy::ZoneGenealogy};

/// Auto terrain.
#[derive(Default)]
pub struct AutoTerrain {
    offset: UVec3,
    genealogy: Arc<ZoneGenealogy>,
}

impl AutoTerrain {
    /// Offset.
    pub fn offset(&self) -> &UVec3 {
        &self.offset
    }

    /// Genealogy.
    pub fn genealogy(&self) -> &Arc<ZoneGenealogy> {
        &self.genealogy
    }
}

impl ClassId for AutoTerrain {
    const CLASS_ID: u32 = 0x03120000;
}

mod read {
    use crate::{
        class::game::ctn::auto_terrain::AutoTerrain,
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for AutoTerrain {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for AutoTerrain {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(1, Self::read_chunk_1)]
        }
    }

    impl AutoTerrain {
        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            self.offset = r.uvec3()?;
            self.genealogy = r.node_ref()?;

            Ok(())
        }
    }
}
