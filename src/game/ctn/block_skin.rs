//! Block skin.

use crate::{Class, FileRef};

/// Block skin.
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
            self.skin = r.file_ref_or_null()?;
            let _parent_skin = r.file_ref_or_null()?;

            Ok(())
        }

        fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            self.skin_effect = r.file_ref_or_null()?;

            Ok(())
        }
    }
}

mod write {
    use std::io::Write;

    use crate::write::{
        writable::{write_body_chunks, WriteBody},
        writer::{IdStateMut, NodeStateMut},
        BodyChunk, BodyChunks, Error, Writer,
    };

    use super::BlockSkin;

    impl WriteBody for BlockSkin {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for BlockSkin {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [
                BodyChunk::normal(2, Self::write_chunk_2),
                BodyChunk::normal(3, Self::write_chunk_3),
            ]
            .into_iter()
        }
    }

    impl BlockSkin {
        fn write_chunk_2<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.file_ref_or_null(self.skin.as_ref())?;
            w.file_ref_or_null(None)?;

            Ok(())
        }

        fn write_chunk_3<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.file_ref_or_null(self.skin_effect.as_ref())?;

            Ok(())
        }
    }
}
