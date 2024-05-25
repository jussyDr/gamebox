use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
};

use super::material::Material;

/// A surface.
#[derive(Default, Debug)]
pub struct Surface;

impl Class for Surface {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 12);
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for Surface {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for Surface {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0900C003,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0900c003(n, d)),
        }]
        .into_iter()
    }
}

impl Surface {
    fn read_chunk_0900c003<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 4
        d.u32()?; // 2
        d.u32()?; // 7
        d.u32()?; // 7
        d.list(|d| {
            d.f32()?;
            d.f32()?;
            d.f32()?;

            Ok(())
        })?;
        d.list(|d| {
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?;
        let b = d
            .list(|d| {
                let a = d.u32()?;
                if a != 0 {
                    if a == 1 {
                        d.node_ref::<Material>()?;
                    } else {
                        d.u32()?;
                    }
                }

                Ok(())
            })?
            .is_empty();
        if !b {
            d.u32()?; // 0
        }
        d.list(|d| {
            d.u16()?;

            Ok(())
        })?;
        d.u32()?; // 0xffffffff

        Ok(())
    }
}
