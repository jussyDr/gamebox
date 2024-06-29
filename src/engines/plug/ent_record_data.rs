//! Types used for reading [EntRecordData] nodes.

use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    read::Reader,
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
};

/// Recorded data of an entity.
#[derive(Default, Debug)]
pub struct EntRecordData;

impl Class for EntRecordData {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 287);
}

impl<R: Read, I, N> ReadBody<R, I, N> for EntRecordData {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        read_body_chunks(self, r)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for EntRecordData {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0911f000,
            read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0(n, r)),
        }]
        .into_iter()
    }
}

impl EntRecordData {
    fn read_chunk_0<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 10
        r.u32()?;
        let size = r.u32()?;
        r.bytes(size as usize)?;

        Ok(())
    }
}
