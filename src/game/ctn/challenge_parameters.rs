use crate::Class;

/// A challenge parameters.
#[derive(Default)]
pub struct ChallengeParameters;

impl Class for ChallengeParameters {
    const CLASS_ID: u32 = 0x0305b000;
}

mod read {
    use std::io::Read;

    use crate::{
        game::ctn::ghost::Ghost,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::ChallengeParameters;

    impl ReadBody for ChallengeParameters {
        fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for ChallengeParameters {
        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::new(1, Self::read_chunk_1),
                BodyChunk::new(4, Self::read_chunk_4),
                BodyChunk::new(8, Self::read_chunk_8),
                BodyChunk::skippable(10, Self::read_chunk_10),
                BodyChunk::new(13, Self::read_chunk_13),
                BodyChunk::skippable(14, Self::read_chunk_14),
            ]
            .into_iter()
        }
    }

    impl ChallengeParameters {
        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _tip = r.string()?;
            let _tip = r.string()?;
            let _tip = r.string()?;
            let _tip = r.string()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _bronze_time = r.u32()?;
            let _silver_time = r.u32()?;
            let _gold_time = r.u32()?;
            let _author_time = r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_8<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _time_limit = r.u32()?;
            let _author_score = r.u32()?;

            Ok(())
        }

        fn read_chunk_10<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _tip = r.string()?;
            let _bronze_time = r.u32()?;
            let _silver_time = r.u32()?;
            let _gold_time = r.u32()?;
            let _author_time = r.u32()?;
            let _time_limit = r.u32()?;
            let _author_score = r.u32()?;

            Ok(())
        }

        fn read_chunk_13(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let _race_validate_ghost = r.internal_node_ref_or_null::<Ghost>()?;

            Ok(())
        }

        fn read_chunk_14<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _map_type = r.string()?;
            let _map_style = r.string()?;
            let _is_validated_for_script_modes = r.bool()?;

            Ok(())
        }
    }
}
