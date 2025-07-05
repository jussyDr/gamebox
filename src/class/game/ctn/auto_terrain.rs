//! Auto terrain.

use crate::ClassId;

/// Auto terrain.
#[derive(Default)]
pub struct AutoTerrain;

impl ClassId for AutoTerrain {
    const CLASS_ID: u32 = 0x03120000;
}

mod read {
    use crate::{
        class::game::ctn::{auto_terrain::AutoTerrain, zone_genealogy::ZoneGenealogy},
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
            let _offset = r.repeat(3, |r| r.u32())?;
            let _genealogy = r.internal_node_ref::<ZoneGenealogy>()?;

            Ok(())
        }
    }
}
