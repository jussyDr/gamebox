//! Types used for reading [Prefab] nodes.

use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read::{
        readable::{read_gbx, HeaderChunkEntry, HeaderChunks, ReadBody, Sealed},
        BodyOptions, HeaderOptions, Readable, Result,
    },
};

use super::static_object_model::StaticObjectModel;

/// Node type corresponding to GameBox files with the extension `Prefab.Gbx`.
#[derive(Default)]
pub struct Prefab;

impl Class for Prefab {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 325);
}

impl Readable for Prefab {}

impl Sealed for Prefab {
    fn read(
        reader: impl Read,
        header_options: HeaderOptions,
        body_options: BodyOptions,
    ) -> Result<Self> {
        read_gbx(reader, header_options, body_options)
    }
}

impl HeaderChunks for Prefab {
    fn header_chunks<R: Read>() -> impl Iterator<Item = HeaderChunkEntry<Self, R>> {
        [].into_iter()
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for Prefab {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 11
        d.u32()?;
        d.u32()?;
        d.string()?; // "\\storage.nadeo.org\graphical_data\Stadium\3D\Items\VTM.max:Fall-------------------------------EXPORT"
        d.u32()?;
        d.list(|d| {
            d.u32()?; // 0
            d.node_ref::<StaticObjectModel>()?;
            d.u32()?; // 0
            d.u32()?; // 0
            d.u32()?; // 0
            d.f32()?; // 1.0
            d.u32()?;
            d.u32()?; // 0
            d.u32()?;
            d.u32()?; // 0xffffffff

            Ok(())
        })?;
        d.u32()?; // 0

        Ok(())
    }
}
