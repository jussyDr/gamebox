use std::{io::Read, rc::Rc};

use crate::{
    common::{Class, ClassId, EngineId},
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
    read::{IdStateMut, NodeStateMut, Reader},
};

use self::graphic::LightBall;

use super::{
    light_user_model::LightUserModel, material_user_inst::MaterialUserInst, surface::Surface,
    visual_indexed_triangles::VisualIndexedTriangles,
};

/// A static object model.
#[derive(Default, Debug)]
pub struct StaticObjectModel {
    solid_to_model: Solid2Model,
}

impl StaticObjectModel {
    pub fn solid_to_model(&self) -> &Solid2Model {
        &self.solid_to_model
    }
}

impl Class for StaticObjectModel {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 345);
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for StaticObjectModel {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        let version = r.u32()?;

        if version != 3 {
            return Err("".into());
        }

        self.solid_to_model = r.unique_internal_node_ref::<Solid2Model>()?;
        if !r.bool8()? {
            r.internal_node_ref_or_null::<Surface>()?;
        }

        Ok(())
    }
}

/// Model from a solir.
#[derive(Default, Debug)]
pub struct Solid2Model {
    layers: Vec<Layer>,
    meshes: Vec<Rc<VisualIndexedTriangles>>,
    materials: Vec<Rc<MaterialUserInst>>,
}

impl Solid2Model {
    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }

    pub fn meshes(&self) -> &[Rc<VisualIndexedTriangles>] {
        &self.meshes
    }

    pub fn materials(&self) -> &[Rc<MaterialUserInst>] {
        &self.materials
    }
}

impl Class for Solid2Model {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 187);
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for Solid2Model {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        read_body_chunks(self, r)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for Solid2Model {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x090bb000,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_090bb000(n, r)),
            },
            BodyChunkEntry {
                id: 0x090bb002,
                read_fn: BodyChunkReadFn::Skippable(|n, r| Self::read_chunk_090bb002(n, r)),
            },
        ]
        .into_iter()
    }
}

impl Solid2Model {
    fn read_chunk_090bb000<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        let version = r.u32()?;

        if !matches!(version, 29..=34) {
            return Err("".into());
        }

        r.null_id()?;
        self.layers = r.list(|r| {
            let mesh_index = r.u32()?;
            let material_index = r.u32()?;
            r.u32()?; // 0xffffffff
            r.u32()?; // 1

            if version >= 32 {
                r.u32()?; // 0
            }

            Ok(Layer {
                mesh_index,
                material_index,
            })
        })?;
        r.u32()?; // 10
        self.meshes = r.list(|r| {
            let visual_indexed_triangles = r.internal_node_ref::<VisualIndexedTriangles>()?;

            Ok(visual_indexed_triangles)
        })?;
        r.u32()?; // 0
        let num_materials = r.u32()?;
        if num_materials == 0 {
            r.u32()?; // 10
            r.list(|r| {
                r.external_node_ref()?;

                Ok(())
            })?;
        }
        r.internal_node_ref_or_null::<Skel>()?;
        r.list(|r| {
            r.f32()?;

            Ok(())
        })?;
        r.u32()?; // 1
        r.u32()?; // 1
        r.u32()?; // 1
        r.u32()?; // 1
        r.f32()?; // 73.47571
        r.u32()?; // 1
        r.f32()?; // 0.011813663
        r.f32()?; // 0.12343697
        r.f32()?; // 0.99153054
        r.f32()?; // 0.98973596
        r.u32()?; // 0xffff7f7f
        r.u32()?; // 0xffff7f7f
        r.u32()?; // 0xffff7f7f
        r.u32()?; // 0xffff7f7f
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0xd05ebb50
        r.u32()?; // 0x01d74f56
        r.u32()?; // 0
        r.string()?; // "Stadium\Media\Material\"
        r.u32()?; // 0
        r.list(|r| {
            r.id_or_null()?; // "?Screen16x9SpotSmall"
            r.u32()?; // 1
            r.node_ref::<Light>()?;
            r.f32()?; // 1.0
            r.u32()?; // 0
            r.u32()?; // 0
            r.u32()?; // 0
            r.f32()?; // 1.0
            r.u32()?; // 0
            r.u32()?; // 0
            r.u32()?; // 0
            r.f32()?; // 1.0
            r.u32()?; // 0
            r.u32()?; // 0xffffffff
            r.u32()?;
            r.u32()?;
            r.u32()?; // 0
            r.u32()?; // 0
            r.u32()?; // 0
            r.u32()?; // 0
            r.u32()?; // 0
            r.u32()?; // 0

            Ok(())
        })?;
        r.list(|r| {
            r.internal_node_ref::<LightUserModel>()?;

            Ok(())
        })?;
        if r.bool32()? {
            r.u32()?; // 0
            r.u32()?; // 0
        }
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 1
        r.string()?; // "NadeoImporter Item Items/palm_trees/big_palm_trees/big_palm_tree_low.Item.xml"
        if version >= 30 {
            r.u32()?;
        }
        self.materials = r.repeat(num_materials as usize, |r| {
            r.u32()?; // 0
            let material = r.internal_node_ref::<MaterialUserInst>()?;

            Ok(material)
        })?;
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0
        r.u32()?; // 0xffffffff
        r.f32()?; // 1.0
        r.f32()?; // 1.0
        r.u32()?; // 0xffffffff
        if version >= 31 {
            r.u32()?; // 0
        }
        if version >= 33 {
            r.u32()?; // 0
        }

        Ok(())
    }

    fn read_chunk_090bb002<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0
        r.u32()?; // 0

        Ok(())
    }
}

#[derive(Default)]
struct Skel;

impl Class for Skel {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 186);
}

impl<R: Read, I: IdStateMut, N> ReadBody<R, I, N> for Skel {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        read_body_chunks(self, r)
    }
}

impl<R: Read, I: IdStateMut, N> BodyChunks<R, I, N> for Skel {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>>
    where
        Self: Sized,
    {
        [BodyChunkEntry {
            id: 0x090ba000,
            read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_0(n, r)),
        }]
        .into_iter()
    }
}

impl Skel {
    fn read_chunk_0<R: Read, I: IdStateMut, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        let version = r.u32()?;

        if version != 19 {
            return Err("".into());
        }

        r.u32()?; // 0xffffffff
        r.u32()?; // 0
        r.u16()?; // 0
        r.u32()?; // 1
        r.id()?; // "Point_001_001_001_001_001_001_006.003"
        r.u16()?; // 0xffff
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.u8()?;

        Ok(())
    }
}

#[derive(Default)]
struct Light;

impl Class for Light {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 29);
}

impl<R: Read, I, N: NodeStateMut> ReadBody<R, I, N> for Light {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        read_body_chunks(self, r)
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
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_3(n, r)),
            },
            BodyChunkEntry {
                id: 0x0901d004,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_4(n, r)),
            },
        ]
        .into_iter()
    }
}

impl Light {
    fn read_chunk_3<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 1
        r.u32()?; // 0xffffffff
        r.f32()?;
        r.f32()?;
        r.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_4<R: Read, I, N: NodeStateMut>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0
        r.internal_node_ref::<LightBall>()?;
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff
        r.u32()?; // 0
        r.u32()?; // 0xffffffff

        Ok(())
    }
}

mod graphic {
    use std::io::Read;

    use crate::{
        common::{Class, ClassId, EngineId},
        read::Reader,
        read::{
            readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
            Result,
        },
    };

    #[derive(Default)]
    struct Light;

    impl Light {
        fn read_chunk_10<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
            r.u32()?; // 0
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;

            Ok(())
        }
    }

    #[derive(Default)]
    struct LightPoint {
        parent: Light,
    }

    impl LightPoint {
        fn read_chunk_4<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
            r.f32()?;
            r.u32()?; // 0

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
        fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
            read_body_chunks(self, r)
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
                    read_fn: BodyChunkReadFn::Normal(|n: &mut Self, r| {
                        Light::read_chunk_10(&mut n.parent.parent, r)
                    }),
                },
                BodyChunkEntry {
                    id: 0x04003004,
                    read_fn: BodyChunkReadFn::Normal(|n: &mut Self, r| {
                        LightPoint::read_chunk_4(&mut n.parent, r)
                    }),
                },
                BodyChunkEntry {
                    id: 0x04002008,
                    read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_8(n, r)),
                },
                BodyChunkEntry {
                    id: 0x04002009,
                    read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_9(n, r)),
                },
                BodyChunkEntry {
                    id: 0x0400200a,
                    read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_10(n, r)),
                },
            ]
            .into_iter()
        }
    }

    impl LightBall {
        fn read_chunk_8<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
            r.u32()?; // 16
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.u32()?; // 0
            r.u32()?; // 0
            r.f32()?;
            r.u32()?; // 0
            r.u32()?; // 0
            r.u32()?; // 0
            r.u32()?; // 0
            r.f32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_9<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
            r.f32()?;

            Ok(())
        }

        fn read_chunk_10<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
            r.f32()?;

            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct Layer {
    mesh_index: u32,
    material_index: u32,
}

impl Layer {
    pub fn mesh_index(&self) -> u32 {
        self.mesh_index
    }

    pub fn material_index(&self) -> u32 {
        self.material_index
    }
}
