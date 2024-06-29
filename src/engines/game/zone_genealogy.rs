use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
    read::{IdStateMut, Reader},
};

/// Zone genealogy.
#[derive(Default, Debug)]
pub struct ZoneGenealogy;

impl Class for ZoneGenealogy {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 285);
}

impl<R: Read, I: IdStateMut, N> ReadBody<R, I, N> for ZoneGenealogy {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        read_body_chunks(self, r)
    }
}

impl<R: Read, I: IdStateMut, N> BodyChunks<R, I, N> for ZoneGenealogy {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0311d002,
            read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0311d002(n, r)),
        }]
        .into_iter()
    }
}

impl ZoneGenealogy {
    fn read_chunk_0311d002<R: Read, I: IdStateMut, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.u32()?; // 1
        r.id()?; // "VoidToGrass"
        r.u32()?; // 0
        r.u32()?; // 0
        r.id()?; // "Grass"

        Ok(())
    }
}
