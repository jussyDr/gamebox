use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateMut},
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
};

#[derive(Default)]
pub struct ZoneGenealogy;

impl Class for ZoneGenealogy {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME, 285);
}

impl<R: Read, I: IdStateMut, N> ReadBody<R, I, N> for ZoneGenealogy {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N> BodyChunks<R, I, N> for ZoneGenealogy {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0311d002,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0311d002(n, d)),
        }]
        .into_iter()
    }
}

impl ZoneGenealogy {
    fn read_chunk_0311d002<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 1
        d.id()?; // "VoidToGrass"
        d.u32()?; // 0
        d.u32()?; // 0
        d.id()?; // "Grass"

        Ok(())
    }
}
