use std::io::Read;

use crate::{
    read::{
        readable::{BodyChunk, BodyChunksInline},
        Error, IdStateMut, Reader,
    },
    Direction,
};

/// A zone genealogy.
#[derive(Default)]
pub struct ZoneGenealogy;

impl BodyChunksInline for ZoneGenealogy {
    fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(2, |n, r| Self::read_chunk_2(n, r), false)];

        chunks.into_iter()
    }
}

impl ZoneGenealogy {
    fn read_chunk_2<N>(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, N>,
    ) -> Result<(), Error> {
        let _zone_ids = r.list(|r| r.id())?;
        let _current_index = r.u32()?;
        let _dir = Direction::read_u32(r)?;
        let _current_zone_id = r.id()?;

        Ok(())
    }
}
