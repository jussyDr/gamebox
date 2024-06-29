//! Types used for reading [Material] nodes.

use std::{io::Read, rc::Rc};

use crate::{
    common::{Class, ClassId, EngineId},
    read::{
        readable::{
            read_body_chunks, read_gbx, BodyChunkEntry, BodyChunkReadFn, BodyChunks,
            HeaderChunkEntry, HeaderChunks, ReadBody, Sealed,
        },
        BodyOptions, HeaderOptions, Readable, Result,
    },
    read::{IdStateMut, NodeStateMut, Reader},
    RcPath,
};

/// Node type corresponding to GameBox files with the extension `Material.Gbx`.
#[derive(Default, Debug)]
pub struct Material {
    custom: Option<Rc<MaterialCustom>>,
}

impl Material {
    pub fn custom(&self) -> Option<&Rc<MaterialCustom>> {
        self.custom.as_ref()
    }
}

impl Class for Material {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 121);
}

impl Readable for Material {}

impl Sealed for Material {
    fn read(
        reader: impl Read,
        header_options: HeaderOptions,
        body_options: BodyOptions,
    ) -> Result<Self> {
        read_gbx(reader, header_options, body_options)
    }
}

impl HeaderChunks for Material {
    fn header_chunks<R: Read>() -> impl Iterator<Item = HeaderChunkEntry<Self, R>> {
        [].into_iter()
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for Material {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        read_body_chunks(self, r)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for Material {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x09079001,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09079001(n, r)),
            },
            BodyChunkEntry {
                id: 0x09079007,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09079007(n, r)),
            },
            BodyChunkEntry {
                id: 0x09079010,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09079010(n, r)),
            },
            BodyChunkEntry {
                id: 0x09079011,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09079011(n, r)),
            },
            BodyChunkEntry {
                id: 0x09079012,
                read_fn: BodyChunkReadFn::Skippable(|n, r| Self::read_chunk_09079012(n, r)),
            },
            BodyChunkEntry {
                id: 0x09079013,
                read_fn: BodyChunkReadFn::Skippable(|n, r| Self::read_chunk_09079013(n, r)),
            },
            BodyChunkEntry {
                id: 0x09079015,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09079015(n, r)),
            },
            BodyChunkEntry {
                id: 0x09079016,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09079016(n, r)),
            },
            BodyChunkEntry {
                id: 0x09079017,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09079017(n, r)),
            },
            BodyChunkEntry {
                id: 0x09079019,
                read_fn: BodyChunkReadFn::Skippable(|n, r| Self::read_chunk_09079019(n, r)),
            },
        ]
        .into_iter()
    }
}

impl Material {
    fn read_chunk_09079001<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_09079007<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        self.custom = r.internal_node_ref_or_null::<MaterialCustom>()?;

        Ok(())
    }

    fn read_chunk_09079010<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.f32()?; // 1.0

        Ok(())
    }

    fn read_chunk_09079011<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09079012<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0
        if self.custom.is_some() {
            r.string()?; // ":data:\Projects\Techno3\Media\Material\Tech3_Block_TDSN_CubeOut.Material.gbx"
            r.u32()?; // 0x7ec30323
            r.u32()?; // 0x803b7649
            r.u32()?; // 0
            r.u32()?; // 0x28002841
            r.u32()?; // 0
            r.u32()?; // 0
            r.u32()?; // 2
            r.u32()?; // 1
            r.f32()?; // 1.0
            r.u32()?; // 0xffffffff
            r.u32()?; // 0
        }

        Ok(())
    }

    fn read_chunk_09079013<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09079015<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 7
        r.u32()?; // 5 | 6 | 0xffffffff
        if r.bool32()? {
            r.u32()?; // 7
        }
        let a = r.u32()?; // 0 | 0xffffffff
        if a == 0 {
            r.u32()?; // 0
        }

        Ok(())
    }

    fn read_chunk_09079016<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0
        r.u32()?; // 0x240

        Ok(())
    }

    fn read_chunk_09079017<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 1
        r.u32()?; // 0x800f0016
        r.f32()?; // -1.0
        r.u32()?; // 0
        r.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09079019<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0
        r.u32()?; // 0

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct MaterialCustom {
    diffuse_texture_path: RcPath,
}

impl MaterialCustom {
    pub fn diffuse_texture_path(&self) -> &RcPath {
        &self.diffuse_texture_path
    }
}

impl Class for MaterialCustom {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 58);
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for MaterialCustom {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        read_body_chunks(self, r)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for MaterialCustom {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x0903a004,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0903a004(n, r)),
            },
            BodyChunkEntry {
                id: 0x0903a00a,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0903a00a(n, r)),
            },
            BodyChunkEntry {
                id: 0x0903a00c,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0903a00c(n, r)),
            },
            BodyChunkEntry {
                id: 0x0903a00f,
                read_fn: BodyChunkReadFn::Skippable(|n, r| Self::read_chunk_0903a00f(n, r)),
            },
            BodyChunkEntry {
                id: 0x0903a011,
                read_fn: BodyChunkReadFn::Skippable(|n, r| Self::read_chunk_0903a011(n, r)),
            },
            BodyChunkEntry {
                id: 0x0903a012,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0903a012(n, r)),
            },
            BodyChunkEntry {
                id: 0x0903a013,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0903a013(n, r)),
            },
            BodyChunkEntry {
                id: 0x0903a014,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0903a014(n, r)),
            },
            BodyChunkEntry {
                id: 0x0903a015,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0903a015(n, r)),
            },
            BodyChunkEntry {
                id: 0x0903a016,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0903a016(n, r)),
            },
        ]
        .into_iter()
    }
}

impl MaterialCustom {
    fn read_chunk_0903a004<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0;

        Ok(())
    }

    fn read_chunk_0903a00a<R: Read, I: IdStateMut, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        if r.bool32()? {
            r.id()?; // "BaseColorTarget"
            r.u32()?; // 3
            r.u32()?; // 1
            r.u32()?; // 0
            r.u32()?; // 0
            r.f32()?; // 1.0
            r.u32()?; // 0
        }

        r.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0903a00c<R: Read, I: IdStateMut, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.list(|r| {
            r.id()?; // "PreLightGen" | "OpacityIsDiffuseAlpha" | "IsPoleEmblem" | "BaseColorHueMask"
            r.u32()?; // 0

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_0903a00f<R: Read, I: IdStateMut, N>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.u32()?; // 2
        r.f32()?; // -1.0
        r.f32()?; // -1.0
        r.u32()?; // 0
        if r.bool32()? {
            r.id()?; // "LodNormal"
            r.u32()?; // 0
        }

        Ok(())
    }

    fn read_chunk_0903a011<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0903a012<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0903a013<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.u32()?; // 0
        r.list(|r| {
            let texture_kind = r.id()?; // "BaseColor" | "RoughMetal" | "Normal" | "BaseColorHueMask"
            r.u32()?; // 0
            let texture_ref = r.external_node_ref()?;
            r.u32()?; // 4
            r.u32()?; // 4

            match texture_kind.as_ref() {
                "ACosSmoothPy" => {}
                "BaseColor" => self.diffuse_texture_path = texture_ref.into(),
                "BaseColorHueMask" => {}
                "Normal" => {}
                "PyBaseColor" => self.diffuse_texture_path = texture_ref.into(),
                "PyNormal" => {}
                "PyRoughMetal" => {}
                "PyX2" => {}
                "PxzBaseColor" => self.diffuse_texture_path = texture_ref.into(),
                "PxzNormal" => {}
                "PxzRoughMetal" => {}
                "RoughMetal" => {}
                _ => return Err(format!("unknown texture kind '{texture_kind}'").into()),
            }

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_0903a014<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 1
        r.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0903a015<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 2
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0903a016<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 2
        r.u32()?; // 0
        r.u32()?; // 0x7f80
        r.u32()?; // 0x0c007800
        r.u32()?; // 0x0x8fff2
        r.u32()?; // 0

        Ok(())
    }
}
