//! Zone genealogy.

use crate::ClassId;

/// Zone genealogy.
#[derive(Default)]
pub struct ZoneGenealogy;

impl ClassId for ZoneGenealogy {
    const CLASS_ID: u32 = 0x0311d000;
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::game::ctn::zone_genealogy::ZoneGenealogy,
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for ZoneGenealogy {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for ZoneGenealogy {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(2, Self::read_chunk_2)]
        }
    }

    impl ZoneGenealogy {
        fn read_chunk_2(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _zone_ids: Vec<Arc<str>> = r.list(|r| r.id())?;
            let _current_index = r.u32()?;
            let _dir = r.u32()?;
            let _current_zone_id: Arc<str> = r.id()?;

            Ok(())
        }
    }
}
