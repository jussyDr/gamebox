use std::io::{Read, Seek};

use crate::{
    classes::visual_indexed_triangles::VisualIndexedTriangles,
    read::{
        deserialize::{Deserializer, IdStateMut, NodeStateMut},
        read_gbx,
        readable::{HeaderChunkEntry, HeaderChunks, Sealed},
        BodyOptions, HeaderOptions, ReadBody, Result,
    },
};

use super::VegetTreeModel;

impl Sealed for VegetTreeModel {
    fn read(
        reader: impl Read + Seek,
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

impl ReadBody for VegetTreeModel {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
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
        d.node::<VisualIndexedTriangles>()?;

        println!("{:02X?}", d.bytes(48)?);

        Ok(())
    }
}
