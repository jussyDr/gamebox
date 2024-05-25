//! Types used for reading [Material] nodes.

use std::{io::Read, rc::Rc};

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read::{
        readable::{
            read_body_chunks, read_gbx, BodyChunkEntry, BodyChunkReadFn, BodyChunks,
            HeaderChunkEntry, HeaderChunks, ReadBody, Sealed,
        },
        BodyOptions, HeaderOptions, Readable, Result,
    },
    RcPath,
};

/// Node type corresponding to GameBox files with the extension `Material.Gbx`.
#[derive(Default, Debug)]
pub struct Material {
    material_custom: Option<Rc<MaterialCustom>>,
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
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for Material {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x09079001,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09079001(n, d)),
            },
            BodyChunkEntry {
                id: 0x09079007,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09079007(n, d)),
            },
            BodyChunkEntry {
                id: 0x09079010,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09079010(n, d)),
            },
            BodyChunkEntry {
                id: 0x09079011,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09079011(n, d)),
            },
            BodyChunkEntry {
                id: 0x09079012,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_09079012(n, d)),
            },
            BodyChunkEntry {
                id: 0x09079013,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_09079013(n, d)),
            },
            BodyChunkEntry {
                id: 0x09079015,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09079015(n, d)),
            },
            BodyChunkEntry {
                id: 0x09079016,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09079016(n, d)),
            },
            BodyChunkEntry {
                id: 0x09079017,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09079017(n, d)),
            },
            BodyChunkEntry {
                id: 0x09079019,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_09079019(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Material {
    fn read_chunk_09079001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_09079007<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        self.material_custom = d.internal_node_ref_or_null::<MaterialCustom>()?;

        Ok(())
    }

    fn read_chunk_09079010<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.f32()?; // 1.0

        Ok(())
    }

    fn read_chunk_09079011<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09079012<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        if self.material_custom.is_some() {
            d.string()?; // ":data:\Projects\Techno3\Media\Material\Tech3_Block_TDSN_CubeOut.Material.gbx"
            d.u32()?; // 0x7ec30323
            d.u32()?; // 0x803b7649
            d.u32()?; // 0
            d.u32()?; // 0x28002841
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 2
            d.u32()?; // 1
            d.f32()?; // 1.0
            d.u32()?; // 0xffffffff
            d.u32()?; // 0
        }

        Ok(())
    }

    fn read_chunk_09079013<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09079015<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 7
        d.u32()?; // 5 | 6 | 0xffffffff
        if d.bool32()? {
            d.u32()?; // 7
        }
        let a = d.u32()?; // 0 | 0xffffffff
        if a == 0 {
            d.u32()?; // 0
        }

        Ok(())
    }

    fn read_chunk_09079016<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0x240

        Ok(())
    }

    fn read_chunk_09079017<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0x800f0016
        d.f32()?; // -1.0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09079019<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

#[derive(Default, Debug)]
struct MaterialCustom {
    diffuse_texture_path: RcPath,
}

impl Class for MaterialCustom {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 58);
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for MaterialCustom {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for MaterialCustom {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x0903a004,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0903a004(n, d)),
            },
            BodyChunkEntry {
                id: 0x0903a00a,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0903a00a(n, d)),
            },
            BodyChunkEntry {
                id: 0x0903a00c,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0903a00c(n, d)),
            },
            BodyChunkEntry {
                id: 0x0903a00f,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0903a00f(n, d)),
            },
            BodyChunkEntry {
                id: 0x0903a011,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_0903a011(n, d)),
            },
            BodyChunkEntry {
                id: 0x0903a012,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0903a012(n, d)),
            },
            BodyChunkEntry {
                id: 0x0903a013,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0903a013(n, d)),
            },
            BodyChunkEntry {
                id: 0x0903a014,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0903a014(n, d)),
            },
            BodyChunkEntry {
                id: 0x0903a015,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0903a015(n, d)),
            },
            BodyChunkEntry {
                id: 0x0903a016,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0903a016(n, d)),
            },
        ]
        .into_iter()
    }
}

impl MaterialCustom {
    fn read_chunk_0903a004<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0;

        Ok(())
    }

    fn read_chunk_0903a00a<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        if d.bool32()? {
            d.id()?; // "BaseColorTarget"
            d.u32()?; // 3
            d.u32()?; // 1
            d.u32()?; // 0
            d.u32()?; // 0
            d.f32()?; // 1.0
            d.u32()?; // 0
        }

        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0903a00c<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.list(|d| {
            d.id()?; // "PreLightGen" | "OpacityIsDiffuseAlpha" | "IsPoleEmblem" | "BaseColorHueMask"
            d.u32()?; // 0

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_0903a00f<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        d.f32()?; // -1.0
        d.f32()?; // -1.0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0903a011<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0903a012<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0903a013<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        d.list(|d| {
            let texture_kind = d.id()?; // "BaseColor" | "RoughMetal" | "Normal" | "BaseColorHueMask"
            d.u32()?; // 0
            let texture_ref = d.external_node_ref()?;
            d.u32()?; // 4
            d.u32()?; // 4

            match texture_kind.as_ref() {
                "BaseColor" => self.diffuse_texture_path = texture_ref.into(),
                "BaseColorHueMask" => {}
                "Normal" => {}
                "RoughMetal" => {}
                _ => return Err("unknown texture kind".into()),
            }

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_0903a014<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0903a015<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0903a016<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        d.u32()?; // 0
        d.u32()?; // 0x7f80
        d.u32()?; // 0x0c007800
        d.u32()?; // 0x0x8fff2
        d.u32()?; // 0

        Ok(())
    }
}
