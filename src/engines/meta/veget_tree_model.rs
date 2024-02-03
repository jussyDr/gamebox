//! Types used for reading [VegetTreeModel] nodes.

use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    engines::plug::visual_indexed_triangles::VisualIndexedTriangles,
    read::{
        readable::{read_gbx, HeaderChunkEntry, HeaderChunks, ReadBody, Sealed},
        BodyOptions, HeaderOptions, Readable, Result,
    },
};

/// Node type corresponding to GameBox files with the extension `VegetTreeModel.Gbx`.
#[derive(Default)]
pub struct VegetTreeModel;

impl Class for VegetTreeModel {
    const CLASS_ID: ClassId = ClassId::new(EngineId::META, 134);
}

impl Readable for VegetTreeModel {}

impl Sealed for VegetTreeModel {
    fn read(
        reader: impl Read,
        header_options: HeaderOptions,
        body_options: BodyOptions,
    ) -> Result<Self> {
        read_gbx(reader, header_options, body_options)
    }
}

impl HeaderChunks for VegetTreeModel {
    fn header_chunks<R: Read>() -> impl Iterator<Item = HeaderChunkEntry<Self, R>> {
        [].into_iter()
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for VegetTreeModel {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 15
        d.u32()?; // 4
        d.u32()?; // 2
        d.u32()?; // 1
        d.u32()?; // 3
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 1
        d.u32()?; // 2
        d.u32()?; // 3
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u8()?; // 0
        d.u32()?; // 4
        d.u32()?; // 5
        d.u32()?; // 6
        d.u32()?; // 7
        d.u32()?; // 8
        d.u32()?; // 9
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u8()?; // 0
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 10
        d.u32()?; // 11
        d.u32()?; // 12
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u32()?; // 0xffffffff
        d.u8()?; // 1
        d.u32()?; // 3
        d.id()?; // "FallTree_bark"
        d.id()?; // "ItemBase_bark"
        d.id()?; // "FallTree_leaf"
        d.u32()?; // 3
        d.u16()?; // 0
        d.internal_node_ref::<VisualIndexedTriangles>()?;
        d.u8()?; // 0
        d.u16()?; // 1
        d.internal_node_ref::<VisualIndexedTriangles>()?;
        d.u8()?; // 0
        d.u16()?; // 2
        d.internal_node_ref::<VisualIndexedTriangles>()?;
        d.u8()?; // 0
        d.u16()?; // 3
        d.u32()?; // 0
        d.internal_node_ref::<VisualIndexedTriangles>()?;
        d.u8()?; // 0
        d.u16()?; // 1
        d.internal_node_ref::<VisualIndexedTriangles>()?;
        d.u8()?; // 0
        d.u16()?; // 2
        d.internal_node_ref::<VisualIndexedTriangles>()?;
        d.u32()?; // 0x48000000
        d.u32()?; // 0x142
        d.u32()?; // 0x20342c8
        d.u32()?; // 0x9d967f90
        d.u32()?; // 0x01d638cf
        d.f32()?; // 1.0
        d.f32()?; // 0.1
        d.u32()?; // 1
        d.u32()?; // 1
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
        d.u32()?; // 25
        d.u32()?; // 26
        d.u32()?; // 2
        d.u32()?; // 2
        d.u32()?; // 0x3e19999a
        d.u32()?; // 0x12c
        d.u32()?; // 14
        d.bytes(116)?;

        Ok(())
    }
}
