use std::io::Read;

use crate::{
    read::{
        readable::{BodyChunk, BodyChunks},
        Reader,
    },
    Error,
};

/// Skin of a [Block](super::Block).
#[derive(Default)]
pub struct BlockSkin;

impl BodyChunks for BlockSkin {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 2] = [
            (2, |n, r| Self::read_chunk_2(n, r), false),
            (3, |n, r| Self::read_chunk_3(n, r), false),
        ];

        chunks.into_iter()
    }
}

impl BlockSkin {
    fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _text = r.string()?;
        let _pack_desc = r.pack_desc()?;
        let _parent_pack_desc = r.pack_desc()?;

        Ok(())
    }

    fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 0 {
            return Err(Error);
        }

        let _foreground_pack_desc = r.pack_desc()?;

        Ok(())
    }
}
