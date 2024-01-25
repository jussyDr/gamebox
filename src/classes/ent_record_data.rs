use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::Deserializer,
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
};

#[derive(Default)]
pub struct EntRecordData;

impl Class for EntRecordData {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 287);
}

impl<R: Read, I, N> ReadBody<R, I, N> for EntRecordData {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for EntRecordData {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0911f000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl EntRecordData {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 10
        d.u32()?;
        let size = d.u32()?;
        d.bytes(size as usize)?;

        Ok(())
    }
}
