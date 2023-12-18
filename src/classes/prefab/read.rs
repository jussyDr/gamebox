use std::io::{Read, Seek};

use crate::read::{
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read_body_chunks, read_gbx,
    readable::{BodyChunkEntry, BodyChunks, HeaderChunkEntry, HeaderChunks, Sealed},
    BodyOptions, HeaderOptions, ReadBody, Result,
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
        read_body_chunks(self, d)
    }
}

impl BodyChunks for Prefab {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [].into_iter()
    }
}
