use std::io::{Read, Seek};

use crate::{
    common::{ClassId, EngineId},
    read::{
        deserialize::{Deserializer, IdStateRef, NodeStateRef},
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
};

use super::{
    item::{ItemMaterial, Mesh},
    visual_indexed_triangles::VisualIndexedTriangles,
};

#[derive(Default)]
pub struct StaticObjectModel;

impl ClassId for StaticObjectModel {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x159;
}

impl ReadBody for StaticObjectModel {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 3
        let _solid_to_model = d.internal_node_ref::<Solid2Model>()?;
        let b = d.bool8()?;
        d.internal_node_ref_or_null::<Surface>()?;
        if b {
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
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0xffffffff
            d.u32()?; // 0
            d.u32()?; // 0xffffffff
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
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
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
        }

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct Solid2Model {
    layers: Vec<(Mesh, ItemMaterial)>,
}

impl ClassId for Solid2Model {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x0bb;
}

impl ReadBody for Solid2Model {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for Solid2Model {
    #[allow(clippy::redundant_closure)]
    fn body_chunks<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
    fn read_chunk_090bb000<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u32()?; // 30 | 34
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
        let num_materials = d.u32()?; // 2
        if num_materials == 0 {
            d.u32()?; // 10
            d.list(|d| {
                d.external_node_ref()?;

                Ok(())
            })?;
        }
        d.u32()?; // 0xffffffff
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
            d.id()?; // "?Screen16x9SpotSmall"
            d.u32()?; // 1
            d.u32()?; // 31
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
            d.u32()?; // 0
            d.u32()?; // 0

            Ok(())
        })?;
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 1
        d.string()?; // "NadeoImporter Item Items/palm_trees/big_palm_trees/big_palm_tree_low.Item.xml"
        d.u32()?; // 1
        d.u32()?; // 0
        let _materials = d.repeat(num_materials as usize, |d| {
            let material = d.internal_node_ref::<MaterialUserInst>()?.clone();
            d.u32()?; // 0

            Ok(material.material.clone())
        })?;
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0xffffffff
        d.f32()?; // 1.0
        d.f32()?; // 1.0
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_090bb002<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct MaterialUserInst {
    material: ItemMaterial,
}

impl ClassId for MaterialUserInst {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x0fd;
}

impl ReadBody for MaterialUserInst {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for MaterialUserInst {
    fn body_chunks<R: Read, I: IdStateRef, N: NodeStateRef>(
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
    fn read_chunk_090fd000<R: Read, I: IdStateRef, N>(
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

    fn read_chunk_090fd001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 5
        d.u32()?; // 0xffffffff
        d.u32()?; // 0
        d.u32()?; // 0
        d.f32()?; // 1.0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_090fd002<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

#[derive(Default)]
struct Surface;

impl ClassId for Surface {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x00c;
}

impl ReadBody for Surface {
    fn read_body<R: Read + Seek, I: IdStateRef, N: NodeStateRef>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for Surface {
    fn body_chunks<R: Read, I: IdStateRef, N: NodeStateRef>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x0900C003,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_0900c003(n, d)),
        }]
        .into_iter()
    }
}

impl Surface {
    fn read_chunk_0900c003<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
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
        d.f32()?; // 1.0
        d.list(|d| {
            d.u32()?;
            d.u32()?;

            Ok(())
        })?;
        d.u32()?; // 0
        d.list(|d| {
            d.u16()?;

            Ok(())
        })?;
        d.u32()?; // 0xffffffff

        Ok(())
    }
}
