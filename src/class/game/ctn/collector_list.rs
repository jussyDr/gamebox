//! Collector list.

use crate::ClassId;

/// Collector list.
#[derive(Default)]
pub struct CollectorList;

impl ClassId for CollectorList {
    const CLASS_ID: u32 = 0x0301b000;
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::game::ctn::collector_list::CollectorList,
        read::{BodyChunk, BodyChunks, BodyReader, Error, ReadBody, read_body_chunks},
    };

    impl ReadBody for CollectorList {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for CollectorList {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl CollectorList {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _collector_stock = r.list(|r| {
                let _block_model: Vec<Option<Arc<str>>> = r.repeat(3, |r| r.id())?;
                let _count = r.u32()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
