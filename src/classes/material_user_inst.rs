use crate::common::{Class, ClassId, EngineId};

use super::item::ItemMaterial;

#[derive(Default, Clone)]
pub struct MaterialUserInst {
    material: ItemMaterial,
}

impl Class for MaterialUserInst {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 253);
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        deserialize::{Deserializer, IdStateMut, NodeStateMut},
        read::{
            readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
            Result,
        },
    };

    use super::MaterialUserInst;

    impl ReadBody for MaterialUserInst {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            d: &mut Deserializer<R, I, N>,
        ) -> Result<()> {
            read_body_chunks(self, d)
        }
    }

    impl BodyChunks for MaterialUserInst {
        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
            [
                BodyChunkEntry {
                    id: 0x090fd000,
                    read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_090fd000(n, d)),
                },
                BodyChunkEntry {
                    id: 0x090fd001,
                    read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_090fd001(n, d)),
                },
                BodyChunkEntry {
                    id: 0x090fd002,
                    read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_090fd002(n, d)),
                },
            ]
            .into_iter()
        }
    }

    impl MaterialUserInst {
        fn read_chunk_090fd000<R: Read, I: IdStateMut, N>(
            &mut self,
            d: &mut Deserializer<R, I, N>,
        ) -> Result<()> {
            d.u32()?; // 11
            let uses_game_material = d.bool8()?;
            d.id_or_null()?; // "TM_wiuehrfsd"
            d.u32()?; // 0xffffffff
            d.u32()?; // 0
            d.u16()?; // 4 | 22
            if uses_game_material {
                let _material_ref = d.string()?;
            } else {
                let _id = d.id()?;
            }
            d.list(|d| {
                d.id()?; // "TargetColor"
                d.id()?; // "Real"
                d.u32()?; // 3

                Ok(())
            })?;
            let _color = d.list(|d| d.u32())?;
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0xffffffff

            Ok(())
        }

        fn read_chunk_090fd001<R: Read, I, N>(
            &mut self,
            d: &mut Deserializer<R, I, N>,
        ) -> Result<()> {
            d.u32()?; // 5
            d.u32()?; // 0xffffffff
            d.u32()?; // 0
            d.u32()?; // 0
            d.f32()?; // 1.0
            d.u32()?; // 0
            d.u32()?; // 0

            Ok(())
        }

        fn read_chunk_090fd002<R: Read, I, N>(
            &mut self,
            d: &mut Deserializer<R, I, N>,
        ) -> Result<()> {
            d.u32()?; // 0
            d.u32()?; // 0

            Ok(())
        }
    }
}

mod write {
    use std::io::Write;

    use crate::{
        serialize::{IdStateMut, NodeStateMut, Serializer},
        write::{writable::WriteBody, Result},
    };

    use super::MaterialUserInst;

    impl WriteBody for MaterialUserInst {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            s: &mut Serializer<W, I, N>,
        ) -> Result {
            Self::write_chunk_0(self, s)?;
            Self::write_chunk_1(self, s)?;
            Self::write_chunk_2(self, s)?;

            Ok(())
        }
    }

    impl MaterialUserInst {
        pub fn write_chunk_0<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
            s.u32(0x090fd000)?;
            s.u32(11)?;
            s.u8(1)?;
            s.u32(0xffffffff)?;
            s.u32(0xffffffff)?;
            s.u32(0)?;
            s.u16(16)?;
            s.string("Stadium\\Media\\Material\\PlatformTech")?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0)?;
            s.u32(0xffffffff)?;

            Ok(())
        }

        pub fn write_chunk_1<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
            s.u32(0x090fd001)?;
            s.u32(5)?;
            s.u32(0xffffffff)?;
            s.u32(0)?;
            s.u32(0)?;
            s.f32(1.0)?;
            s.u32(0)?;
            s.u32(0)?;

            Ok(())
        }

        pub fn write_chunk_2<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
            s.u32(0x090fd002)?;
            s.u32(0)?;
            s.u32(0)?;

            Ok(())
        }
    }
}
