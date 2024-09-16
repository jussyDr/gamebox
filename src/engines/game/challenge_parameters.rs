use std::io::Read;

use crate::{
    read::{
        readable::{BodyChunk, BodyChunks},
        IdStateMut, NodeStateMut, Reader,
    },
    Error,
};

use super::ghost::Ghost;

/// Parameters of a [Challenge](super::Challenge).
#[derive(Default)]
pub struct ChallengeParameters;

impl BodyChunks for ChallengeParameters {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 6] = [
            (1, |n, r| Self::read_chunk_1(n, r), false),
            (4, |n, r| Self::read_chunk_4(n, r), false),
            (8, |n, r| Self::read_chunk_8(n, r), false),
            (10, |n, r| Self::read_chunk_10(n, r), true),
            (13, |n, r| Self::read_chunk_13(n, r), false),
            (14, |n, r| Self::read_chunk_14(n, r), true),
        ];

        chunks.into_iter()
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
        let _race_validate_ghost = r.node::<Ghost>()?;

        Ok(())
    }

    fn read_chunk_14<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _map_type = r.string()?;
        let _map_style = r.string()?;
        let _is_validated_for_script_modes = r.bool()?;

        Ok(())
    }
}
