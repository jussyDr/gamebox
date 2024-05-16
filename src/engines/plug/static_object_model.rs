use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
};

use self::graphic::LightBall;

use super::{
    light_user_model::LightUserModel, material_user_inst::MaterialUserInst, surface::Surface,
    visual_indexed_triangles::VisualIndexedTriangles,
};

/// A static object model.
#[derive(Default)]
pub struct StaticObjectModel {
    solid_to_model: Solid2Model,
}

impl Class for StaticObjectModel {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 345);
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for StaticObjectModel {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let version = d.u32()?;

        if version != 3 {
            return Err("".into());
        }

        self.solid_to_model = d.unique_internal_node_ref::<Solid2Model>()?;
        if !d.bool8()? {
            d.internal_node_ref_or_null::<Surface>()?;
        }

        Ok(())
    }
}

/// Model from a solid.
#[derive(Default, Clone)]
pub struct Solid2Model;

impl Class for Solid2Model {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 187);
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for Solid2Model {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for Solid2Model {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x090bb000,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_090bb000(n, d)),
            },
            BodyChunkEntry {
                id: 0x090bb002,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_090bb002(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Solid2Model {
    fn read_chunk_090bb000<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u32()?;

        if !matches!(version, 29..=34) {
            return Err("".into());
        }

        d.null_id()?;
        let _layers = d.list(|d| {
            let mesh_index = d.u32()?;
            let material_index = d.u32()?;
            d.u32()?; // 0xffffffff
            d.u32()?; // 1

            if version >= 32 {
                d.u32()?; // 0
            }

            Ok((mesh_index, material_index))
        })?;
        d.u32()?; // 10
        let _meshes = d.list(|d| {
            let _visual_indexed_triangles = d.internal_node_ref::<VisualIndexedTriangles>()?;

            Ok(())
        })?;
        d.u32()?; // 0
        let num_materials = d.u32()?;
        if num_materials == 0 {
            d.u32()?; // 10
            d.list(|d| {
                d.external_node_ref()?;

                Ok(())
            })?;
        }
        d.internal_node_ref_or_null::<Skel>()?;
        d.list(|d| {
            d.f32()?;

            Ok(())
        })?;
        d.u32()?; // 1
        d.u32()?; // 1
        d.u32()?; // 1
        d.u32()?; // 1
        d.f32()?; // 73.47571
        d.u32()?; // 1
        d.f32()?; // 0.011813663
        d.f32()?; // 0.12343697
        d.f32()?; // 0.99153054
        d.f32()?; // 0.98973596
        d.u32()?; // 0xffff7f7f
        d.u32()?; // 0xffff7f7f
        d.u32()?; // 0xffff7f7f
        d.u32()?; // 0xffff7f7f
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xd05ebb50
        d.u32()?; // 0x01d74f56
        d.u32()?; // 0
        d.string()?; // "Stadium\Media\Material\"
        d.u32()?; // 0
        d.list(|d| {
            d.id_or_null()?; // "?Screen16x9SpotSmall"
            d.u32()?; // 1
            d.node_ref::<Light>()?;
            d.f32()?; // 1.0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.f32()?; // 1.0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.f32()?; // 1.0
            d.u32()?; // 0
            d.u32()?; // 0xffffffff
            d.u32()?;
            d.u32()?;
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0

            Ok(())
        })?;
        d.list(|d| {
            d.internal_node_ref::<LightUserModel>()?;

            Ok(())
        })?;
        if d.bool32()? {
            d.u32()?; // 0
            d.u32()?; // 0
        }
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 1
        d.string()?; // "NadeoImporter Item Items/palm_trees/big_palm_trees/big_palm_tree_low.Item.xml"
        if version >= 30 {
            d.u32()?;
        }
        let _materials = d.repeat(num_materials as usize, |d| {
            d.u32()?; // 0
            d.internal_node_ref::<MaterialUserInst>()?;

            Ok(())
        })?;
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.f32()?; // 1.0
        d.f32()?; // 1.0
        d.u32()?; // 0xffffffff
        if version >= 31 {
            d.u32()?; // 0
        }
        if version >= 33 {
            d.u32()?; // 0
        }

        Ok(())
    }

    fn read_chunk_090bb002<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

#[derive(Default)]
struct Skel;

impl Class for Skel {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 186);
}

impl<R: Read, I: IdStateMut, N> ReadBody<R, I, N> for Skel {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N> BodyChunks<R, I, N> for Skel {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>>
    where
        Self: Sized,
    {
        [BodyChunkEntry {
            id: 0x090ba000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0(n, d)),
        }]
        .into_iter()
    }
}

impl Skel {
    fn read_chunk_0<R: Read, I: IdStateMut, N>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u32()?;

        if version != 19 {
            return Err("".into());
        }

        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u16()?; // 0
        d.u32()?; // 1
        d.id()?; // "Point_001_001_001_001_001_001_006.003"
        d.u16()?; // 0xffff
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u32()?;
        d.u8()?;

        Ok(())
    }
}

#[derive(Default)]
struct Light;

impl Class for Light {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 29);
}

impl<R: Read, I, N: NodeStateMut> ReadBody<R, I, N> for Light {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N: NodeStateMut> BodyChunks<R, I, N> for Light {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>>
    where
        Self: Sized,
    {
        [
            BodyChunkEntry {
                id: 0x0901d003,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_3(n, d)),
            },
            BodyChunkEntry {
                id: 0x0901d004,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_4(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Light {
    fn read_chunk_3<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        d.u32()?; // 0xffffffff
        d.f32()?;
        d.f32()?;
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_4<R: Read, I, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        d.internal_node_ref::<LightBall>()?;
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0xffffffff

        Ok(())
    }
}

mod graphic {
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
    struct Light;

    impl Light {
        fn read_chunk_10<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
            d.u32()?; // 0
            d.f32()?;
            d.f32()?;
            d.f32()?;
            d.u32()?;
            d.u32()?;
            d.f32()?;
            d.f32()?;
            d.f32()?;
            d.f32()?;
            d.f32()?;
            d.f32()?;

            Ok(())
        }
    }

    #[derive(Default)]
    struct LightPoint {
        parent: Light,
    }

    impl LightPoint {
        fn read_chunk_4<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
            d.f32()?;
            d.u32()?; // 0

            Ok(())
        }
    }

    #[derive(Default)]
    pub struct LightBall {
        parent: LightPoint,
    }

    impl Class for LightBall {
        const CLASS_ID: ClassId = ClassId::new(EngineId::GRAPHIC, 2);
    }

    impl<R: Read, I, N> ReadBody<R, I, N> for LightBall {
        fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
            read_body_chunks(self, d)
        }
    }

    impl<R: Read, I, N> BodyChunks<R, I, N> for LightBall {
        fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>>
        where
            Self: Sized,
        {
            [
                BodyChunkEntry {
                    id: 0x0400100a,
                    read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                        Light::read_chunk_10(&mut n.parent.parent, d)
                    }),
                },
                BodyChunkEntry {
                    id: 0x04003004,
                    read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                        LightPoint::read_chunk_4(&mut n.parent, d)
                    }),
                },
                BodyChunkEntry {
                    id: 0x04002008,
                    read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_8(n, d)),
                },
                BodyChunkEntry {
                    id: 0x04002009,
                    read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_9(n, d)),
                },
                BodyChunkEntry {
                    id: 0x0400200a,
                    read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_10(n, d)),
                },
            ]
            .into_iter()
        }
    }

    impl LightBall {
        fn read_chunk_8<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
            d.u32()?; // 16
            d.f32()?;
            d.f32()?;
            d.f32()?;
            d.f32()?;
            d.u32()?; // 0
            d.u32()?; // 0
            d.f32()?;
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.f32()?;
            d.u32()?;

            Ok(())
        }

        fn read_chunk_9<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
            d.f32()?;

            Ok(())
        }

        fn read_chunk_10<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
            d.f32()?;

            Ok(())
        }
    }
}
