//! Challenge parameters.

use std::sync::Arc;

use crate::Class;

use super::Ghost;

/// Challenge parameters.
#[derive(Default)]
pub struct ChallengeParameters {
    bronze_time: u32,
    silver_time: u32,
    gold_time: u32,
    author_time: u32,
    time_limit: u32,
    author_score: u32,
    validation_ghost: Option<Arc<Ghost>>,
}

impl Class for ChallengeParameters {
    const CLASS_ID: u32 = 0x0305b000;
}

impl ChallengeParameters {
    /// Bronze time.
    pub const fn bronze_time(&self) -> u32 {
        self.bronze_time
    }

    /// Silver time.
    pub const fn silver_time(&self) -> u32 {
        self.silver_time
    }

    /// Gold time.
    pub const fn gold_time(&self) -> u32 {
        self.gold_time
    }

    /// Author time.
    pub const fn author_time(&self) -> u32 {
        self.author_time
    }

    /// Time limit.
    pub const fn time_limit(&self) -> u32 {
        self.time_limit
    }

    /// Author score.
    pub const fn author_score(&self) -> u32 {
        self.author_score
    }

    /// Validation ghost.
    pub const fn validation_ghost(&self) -> Option<&Arc<Ghost>> {
        self.validation_ghost.as_ref()
    }
}

mod read {
    use std::io::{Read, Seek};

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
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for ChallengeParameters {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(1, Self::read_chunk_1),
                BodyChunk::normal(4, Self::read_chunk_4),
                BodyChunk::normal(8, Self::read_chunk_8),
                BodyChunk::skippable(10, Self::read_chunk_10),
                BodyChunk::normal(13, Self::read_chunk_13),
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
            self.bronze_time = r.u32()?;
            self.silver_time = r.u32()?;
            self.gold_time = r.u32()?;
            self.author_time = r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_8<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.time_limit = r.u32()?;
            self.author_score = r.u32()?;

            Ok(())
        }

        fn read_chunk_10<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _tip = r.string()?;
            self.bronze_time = r.u32()?;
            self.silver_time = r.u32()?;
            self.gold_time = r.u32()?;
            self.author_time = r.u32()?;
            self.time_limit = r.u32()?;
            self.author_score = r.u32()?;

            Ok(())
        }

        fn read_chunk_13(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.validation_ghost = r.internal_node_ref_or_null::<Ghost>()?;

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
