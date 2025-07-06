//! Ent record data.

use crate::ClassId;

/// Ent record data.
#[derive(Default)]
pub struct EntRecordData;

impl ClassId for EntRecordData {
    const CLASS_ID: u32 = 0x0911f000;
}

mod read {
    use crate::{
        class::plug::ent_record_data::EntRecordData,
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
    };

    impl ReadBody for EntRecordData {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for EntRecordData {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl EntRecordData {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 10 {
                return Err(error_unknown_chunk_version(version));
            }

            let _uncompressed_size = r.u32()?;
            let _compressed_data = r.byte_buf()?;

            Ok(())
        }
    }
}
