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
    use std::io::Read;

    use crate::{
        deserialize::{Deserializer, IdStateMut},
        read::{
            readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
            Result,
        },
    };

    use super::MaterialUserInst;

    impl<R: Read, I: IdStateMut, N> ReadBody<R, I, N> for MaterialUserInst {
        fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
            read_body_chunks(self, d)
        }
    }

    impl<R: Read, I: IdStateMut, N> BodyChunks<R, I, N> for MaterialUserInst {
        fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
            let version = d.u32()?;

            if !matches!(version, 9..=11) {
                return Err("".into());
            }

            let uses_game_material = if version >= 11 { d.bool8()? } else { true };

            d.id_or_null()?; // "TM_wiuehrfsd"
            d.u32()?; // 0xffffffff
            d.u32()?; // 0
            d.u8()?;
            if version >= 10 {
                d.u8()?;
            }
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
        common::END_OF_NODE_MARKER,
        serialize::Serializer,
        write::{writable::WriteBody, Result},
    };

    use super::MaterialUserInst;

    impl<W: Write, I, N> WriteBody<W, I, N> for MaterialUserInst {
        fn write_body(&self, s: &mut Serializer<W, I, N>) -> Result {
            Self::write_chunk_0(self, s)?;
            Self::write_chunk_1(self, s)?;
            Self::write_chunk_2(self, s)?;

            s.u32(END_OF_NODE_MARKER)?;

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
