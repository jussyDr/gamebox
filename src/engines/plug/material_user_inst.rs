//! Types used for reading [MaterialUserInst] nodes.

use std::rc::Rc;

use crate::{
    common::{Class, ClassId, EngineId},
    Rgb,
};

/// A user-made material.
#[derive(Default, Debug)]
pub struct MaterialUserInst {
    material: Material,
}

impl MaterialUserInst {
    pub fn material(&self) -> &Material {
        &self.material
    }
}

#[derive(Debug)]
pub enum Material {
    Game { path: String },
    Custom { id: Rc<str>, color: Rgb },
}

impl Default for Material {
    fn default() -> Self {
        Self::Game {
            path: String::new(),
        }
    }
}

impl Class for MaterialUserInst {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 253);
}

mod read {
    use std::io::Read;

    use crate::{
        read::{
            readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
            Result,
        },
        read::{IdStateMut, Reader},
        Rgb,
    };

    use super::{Material, MaterialUserInst};

    impl<R: Read, I: IdStateMut, N> ReadBody<R, I, N> for MaterialUserInst {
        fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
            read_body_chunks(self, r)
        }
    }

    impl<R: Read, I: IdStateMut, N> BodyChunks<R, I, N> for MaterialUserInst {
        fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
            [
                BodyChunkEntry {
                    id: 0x090fd000,
                    read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_090fd000(n, r)),
                },
                BodyChunkEntry {
                    id: 0x090fd001,
                    read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_090fd001(n, r)),
                },
                BodyChunkEntry {
                    id: 0x090fd002,
                    read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_090fd002(n, r)),
                },
            ]
            .into_iter()
        }
    }

    impl MaterialUserInst {
        fn read_chunk_090fd000<R: Read, I: IdStateMut, N>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<()> {
            let version = r.u32()?;

            if !matches!(version, 9..=11) {
                return Err("".into());
            }

            let uses_game_material = if version >= 11 { r.bool8()? } else { true };

            r.id_or_null()?; // "TM_wiuehrfsd"
            r.u32()?; // 0xffffffff
            r.u32()?; // 0
            r.u8()?;
            if version >= 10 {
                r.u8()?;
            }
            if uses_game_material {
                let path = r.string()?;
                self.material = Material::Game { path };
            } else {
                let id = r.id()?;
                self.material = Material::Custom {
                    id,
                    color: Rgb { r: 0, g: 0, b: 0 },
                };
            }
            r.list(|r| {
                r.id()?; // "TargetColor"
                r.id()?; // "Real"
                r.u32()?; // 3

                Ok(())
            })?;
            let color_values = r.list(|r| r.u32())?;
            r.u32()?; // 0
            r.u32()?; // 0
            r.u32()?; // 0
            r.u32()?; // 0xffffffff

            if !color_values.is_empty() {
                if let Material::Custom { color, .. } = &mut self.material {
                    *color = Rgb {
                        r: color_values[0] as u8,
                        g: color_values[1] as u8,
                        b: color_values[2] as u8,
                    };
                }
            }

            Ok(())
        }

        fn read_chunk_090fd001<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
            r.u32()?; // 5
            r.u32()?; // 0xffffffff
            r.u32()?; // 0
            r.u32()?; // 0
            r.f32()?; // 1.0
            r.u32()?; // 0
            r.u32()?; // 0

            Ok(())
        }

        fn read_chunk_090fd002<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
            r.u32()?; // 0
            r.u32()?; // 0

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
        fn write_chunk_0<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
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

        fn write_chunk_1<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
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

        fn write_chunk_2<W: Write, I, N>(&self, s: &mut Serializer<W, I, N>) -> Result {
            s.u32(0x090fd002)?;
            s.u32(0)?;
            s.u32(0)?;

            Ok(())
        }
    }
}
