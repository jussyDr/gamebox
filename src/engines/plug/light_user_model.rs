//! Types used for reading [LightUserModel] nodes.

use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    read::Reader,
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
};

/// A user-made light model.
#[derive(Default, Debug)]
pub struct LightUserModel;

impl Class for LightUserModel {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 249);
}

impl<R: Read, I, N> ReadBody<R, I, N> for LightUserModel {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        read_body_chunks(self, r)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for LightUserModel {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>>
    where
        Self: Sized,
    {
        [BodyChunkEntry {
            id: 0x090f9000,
            read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0(n, r)),
        }]
        .into_iter()
    }
}

impl LightUserModel {
    fn read_chunk_0<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        let version = r.u32()?;

        if version != 1 {
            return Err("".into());
        }

        r.u32()?; // 0
        r.f32()?;
        r.f32()?;
        r.f32()?;
        r.f32()?;
        r.f32()?;
        r.f32()?;
        r.f32()?;
        r.f32()?;
        r.f32()?;
        r.f32()?;
        r.f32()?;
        r.f32()?;

        Ok(())
    }
}
