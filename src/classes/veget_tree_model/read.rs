use std::io::{Read, Seek};

use crate::read::{
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read_gbx,
    readable::{HeaderChunkEntry, HeaderChunks, Sealed},
    BodyOptions, HeaderOptions, ReadBody, Result,
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
        println!("{:02X?}", d.bytes(48)?);

        Ok(())
    }
}
