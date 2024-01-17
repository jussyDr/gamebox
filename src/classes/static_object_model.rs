use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
};

use super::{
    material_user_inst::MaterialUserInst, visual_indexed_triangles::VisualIndexedTriangles,
};

#[derive(Default)]
pub struct StaticObjectModel;

impl Class for StaticObjectModel {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 345);
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for StaticObjectModel {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
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

        if !matches!(version, 29 | 30 | 34) {
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

            Ok(())
        })?;
        d.u32()?; // 0
        d.u32()?; // 0
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
struct Surface;

impl Class for Surface {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 12);
}

impl<R: Read, I, N> ReadBody<R, I, N> for Surface {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for Surface {
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
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
