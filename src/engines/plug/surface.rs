use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
    read::{IdStateMut, NodeStateMut, Reader},
};

use super::material::Material;

/// A surface.
#[derive(Default, Debug)]
pub struct Surface;

impl Class for Surface {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 12);
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for Surface {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        read_body_chunks(self, r)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for Surface {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0900C003,
            read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0900c003(n, r)),
        }]
        .into_iter()
    }
}

impl Surface {
    fn read_chunk_0900c003<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.u32()?; // 4
        r.u32()?; // 2
        r.u32()?; // 7
        r.u32()?; // 7
        r.list(|r| {
            r.f32()?;
            r.f32()?;
            r.f32()?;

            Ok(())
        })?;
        r.list(|r| {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        })?;
        r.u32()?; // 0
        r.u32()?; // 0
        r.f32()?;
        let b = r
            .list(|r| {
                let a = r.u32()?;
                if a != 0 {
                    if a == 1 {
                        r.node_ref::<Material>()?;
                    } else {
                        r.u32()?;
                    }
                }

                Ok(())
            })?
            .is_empty();
        if !b {
            r.u32()?; // 0
        }
        r.list(|r| {
            r.u16()?;

            Ok(())
        })?;
        r.u32()?; // 0xffffffff

        Ok(())
    }
}
