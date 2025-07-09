//! Media block time.

use crate::ClassId;

/// Media block time.
#[derive(Default)]
pub struct MediaBlockTime;

impl ClassId for MediaBlockTime {
    const CLASS_ID: u32 = 0x03085000;
}

mod read {
    use crate::{
        class::game::ctn::media_block_time::MediaBlockTime,
        read::{BodyChunk, BodyChunks, BodyReader, Error, ReadBody, read_body_chunks},
    };

    impl ReadBody for MediaBlockTime {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaBlockTime {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl MediaBlockTime {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _keys = r.list(|r| {
                let _time = r.f32()?;
                let _time_value = r.f32()?;
                let _tangent = r.f32()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
