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
    use std::io::Read;

    use crate::{
        class::game::challenge::Challenge,
        read::{
            BodyChunk, BodyChunks, Error, HeaderChunk, HeaderChunks, ReadBody, Readable,
            error_unknown_chunk_version, read_body_chunks,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl Readable for Challenge {}

    impl HeaderChunks for Challenge {
        fn header_chunks<R: Read, I, N>() -> impl IntoIterator<Item = HeaderChunk<Self, R, I, N>> {
            [HeaderChunk::new(2, Self::read_chunk_2)]
        }
    }

    impl ReadBody for Challenge {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Challenge {
        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
            []
        }
    }

    impl Challenge {
        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u8()?;

            if version != 13 {
                return Err(error_unknown_chunk_version(version as u32));
            }

            let need_unlock = r.bool32()?;
            let bronze_time = r.u32()?;
            let silver_time = r.u32()?;
            let gold_time = r.u32()?;
            let author_time = r.u32()?;
            let cost = r.u32()?;
            let is_lap_race = r.bool32()?;
            let mode = r.u32()?;
            r.u32()?;
            let author_score = r.u32()?;
            let editor = r.u32()?;
            r.u32()?;
            let num_checkpoints = r.u32()?;
            let num_laps = r.u32()?;

            Ok(())
        }
    }
}
