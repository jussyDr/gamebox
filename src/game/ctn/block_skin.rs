//! Block skin.

use crate::{Class, FileRef};

/// A block skin.
#[derive(Default, Debug)]
pub struct BlockSkin {
    skin: Option<FileRef>,
    skin_effect: Option<FileRef>,
}

impl Class for BlockSkin {
    const CLASS_ID: u32 = 0x03059000;
}

impl BlockSkin {
    /// Skin.
    pub const fn skin(&self) -> Option<&FileRef> {
        self.skin.as_ref()
    }

    /// Skin effect.
    pub const fn skin_effect(&self) -> Option<&FileRef> {
        self.skin_effect.as_ref()
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::BlockSkin;

    impl ReadBody for BlockSkin {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for BlockSkin {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(2, Self::read_chunk_2),
                BodyChunk::normal(3, Self::read_chunk_3),
            ]
            .into_iter()
        }
    }

    impl BlockSkin {
        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _text = r.string()?;
            self.skin = r.pack_desc_or_null()?;
            let _parent_skin = r.pack_desc_or_null()?;

            Ok(())
        }

        fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            self.skin_effect = r.pack_desc_or_null()?;

            Ok(())
        }
    }
}
