//! Challenge

use crate::{ClassId, SubExtensions};

/// A challenge.
#[derive(Default)]
pub struct Challenge;

impl ClassId for Challenge {
    const CLASS_ID: u32 = 0x03043000;
}

impl SubExtensions for Challenge {
    const SUB_EXTENSIONS: &[&str] = &["Map"];
}

mod read {
    use crate::{
        class::game::challenge::Challenge,
        read::{
            BodyChunk, BodyChunks, Error, HeaderChunk, HeaderChunks, ReadBody, Readable,
            error_unknown_chunk_version, read_body_chunks,
            reader::{BodyReader, HeaderReader},
        },
    };

    impl Readable for Challenge {}

    impl HeaderChunks for Challenge {
        fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>> {
            [HeaderChunk::new(2, Self::read_chunk_2)]
        }
    }

    impl ReadBody for Challenge {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Challenge {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            []
        }
    }

    impl Challenge {
        fn read_chunk_2(&mut self, r: &mut impl HeaderReader) -> Result<(), Error> {
            let version = r.u8()?;

            if version != 13 {
                return Err(error_unknown_chunk_version(version as u32));
            }

            let _need_unlock = r.bool32()?;
            let _bronze_time = r.u32()?;
            let _silver_time = r.u32()?;
            let _gold_time = r.u32()?;
            let _author_time = r.u32()?;
            let _cost = r.u32()?;
            let _is_lap_race = r.bool32()?;
            let _mode = r.u32()?;
            r.u32()?;
            let _author_score = r.u32()?;
            let _editor = r.u32()?;
            r.u32()?;
            let _num_checkpoints = r.u32()?;
            let _num_laps = r.u32()?;

            Ok(())
        }
    }
}
