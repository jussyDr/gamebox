//! Types used for reading [VegetTreeModel] nodes.

use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    engines::plug::visual_indexed_triangles::VisualIndexedTriangles,
    read::{
        readable::{read_gbx, HeaderChunkEntry, HeaderChunks, ReadBody, Sealed},
        BodyOptions, HeaderOptions, Readable, Result,
    },
    read::{IdStateMut, NodeStateMut, Reader},
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
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 15
        r.u32()?; // 4
        r.u32()?; // 2
        r.u32()?; // 1
        r.u32()?; // 3
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff
        r.u32()?; // 1
        r.u32()?; // 2
        r.u32()?; // 3
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff
        r.u8()?; // 0
        r.u32()?; // 4
        r.u32()?; // 5
        r.u32()?; // 6
        r.u32()?; // 7
        r.u32()?; // 8
        r.u32()?; // 9
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff
        r.u8()?; // 0
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff
        r.u32()?; // 10
        r.u32()?; // 11
        r.u32()?; // 12
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff
        r.u32()?; // 0xffffffff
        r.u8()?; // 1
        r.u32()?; // 3
        r.id()?; // "FallTree_bark"
        r.id()?; // "ItemBase_bark"
        r.id()?; // "FallTree_leaf"
        r.u32()?; // 3
        r.u16()?; // 0
        r.internal_node_ref::<VisualIndexedTriangles>()?;
        r.u8()?; // 0
        r.u16()?; // 1
        r.internal_node_ref::<VisualIndexedTriangles>()?;
        r.u8()?; // 0
        r.u16()?; // 2
        r.internal_node_ref::<VisualIndexedTriangles>()?;
        r.u8()?; // 0
        r.u16()?; // 3
        r.u32()?; // 0
        r.internal_node_ref::<VisualIndexedTriangles>()?;
        r.u8()?; // 0
        r.u16()?; // 1
        r.internal_node_ref::<VisualIndexedTriangles>()?;
        r.u8()?; // 0
        r.u16()?; // 2
        r.internal_node_ref::<VisualIndexedTriangles>()?;
        r.u32()?; // 0x48000000
        r.u32()?; // 0x142
        r.u32()?; // 0x20342c8
        r.u32()?; // 0x9d967f90
        r.u32()?; // 0x01d638cf
        r.f32()?; // 1.0
        r.f32()?; // 0.1
        r.u32()?; // 1
        r.u32()?; // 1
        r.u32()?; // 7
        r.u32()?; // 7
        r.list(|r| {
            r.f32()?;
            r.f32()?;
            r.f32()?;

            Ok(())
        })?;
        r.list(|r| {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        })?;
        r.u32()?; // 0
        r.u32()?; // 25
        r.u32()?; // 26
        r.u32()?; // 2
        r.u32()?; // 2
        r.u32()?; // 0x3e19999a
        r.u32()?; // 0x12c
        r.u32()?; // 14
        r.bytes(116)?;

        Ok(())
    }
}
