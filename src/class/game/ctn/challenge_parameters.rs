//! Challenge parameters.

use crate::ClassId;

/// Challenge parameters.
#[derive(Default)]
pub struct ChallengeParameters;

impl ClassId for ChallengeParameters {
    const CLASS_ID: u32 = 0x0305b000;
}

mod read {
    use crate::{
        class::game::ctn::{challenge_parameters::ChallengeParameters, ghost::Ghost},
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for ChallengeParameters {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for ChallengeParameters {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(1, Self::read_chunk_1),
                BodyChunk::new(4, Self::read_chunk_4),
                BodyChunk::new(8, Self::read_chunk_8),
                BodyChunk::skippable(10, Self::read_chunk_10),
                BodyChunk::new(13, Self::read_chunk_13),
                BodyChunk::skippable(14, Self::read_chunk_14),
            ]
        }
    }

    impl ChallengeParameters {
        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _tip_1 = r.string()?;
            let _tip_2 = r.string()?;
            let _tip_3 = r.string()?;
            let _tip_4 = r.string()?;

            Ok(())
        }

        fn read_chunk_4(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _bronze_time = r.u32()?;
            let _silver_time = r.u32()?;
            let _gold_time = r.u32()?;
            let _author_time = r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_8(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _time_limit = r.u32()?;
            let _author_score = r.u32()?;

            Ok(())
        }

        fn read_chunk_10(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _tip = r.string()?;
            let _bronze_time = r.u32()?;
            let _silver_time = r.u32()?;
            let _gold_time = r.u32()?;
            let _author_time = r.u32()?;
            let _time_limit = r.u32()?;
            let _author_score = r.u32()?;

            Ok(())
        }

        fn read_chunk_13(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _race_validate_ghost = r.internal_node_ref_or_null::<Ghost>()?;

            Ok(())
        }

        fn read_chunk_14(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _map_type = r.string()?;
            let _map_style = r.string()?;
            let _is_validated_for_script_modes = r.bool32()?;

            Ok(())
        }
    }
}
