use std::io::{Read, Seek};

use crate::{
    classes::static_object_model::StaticObjectModel,
    read::{
        deserialize::{Deserializer, IdStateMut, NodeStateMut},
        read_gbx,
        readable::{HeaderChunkEntry, HeaderChunks, Sealed},
        BodyOptions, HeaderOptions, ReadBody, Result,
    },
};

use super::Prefab;

impl Sealed for Prefab {
    fn read(
        reader: impl Read + Seek,
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

impl ReadBody for Prefab {
    fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 11
        d.u32()?;
        d.u32()?;
        d.string()?; // "\\storage.nadeo.org\graphical_data\Stadium\3D\Items\VTM.max:Fall-------------------------------EXPORT"
        d.u32()?;
        d.list(|d| {
            d.u32()?; // 0
            d.node::<StaticObjectModel>()?;
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
