//! Block info clip.

use std::ops::Deref;

use crate::Class;

use super::block_info::BlockInfo;

/// A block info clip.
#[derive(Default)]
pub struct BlockInfoClip {
    parent: BlockInfo,
}

impl Class for BlockInfoClip {
    const CLASS_ID: u32 = 0x03053000;
}

impl Deref for BlockInfoClip {
    type Target = BlockInfo;

    fn deref(&self) -> &BlockInfo {
        &self.parent
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks, readable,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody, Readable,
    };

    use self::readable::{HeaderChunk, HeaderChunks};

    use super::BlockInfoClip;

    impl Readable for BlockInfoClip {}

    impl readable::Sealed for BlockInfoClip {}

    impl HeaderChunks for BlockInfoClip {
        fn parent(&mut self) -> Option<&mut impl HeaderChunks> {
            Some(&mut self.parent)
        }

        fn header_chunks<R: Read, I: IdStateMut, N>(
        ) -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

    impl ReadBody for BlockInfoClip {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for BlockInfoClip {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>>
        {
            [
                BodyChunk::normal(2, Self::read_chunk_2),
                BodyChunk::normal(4, Self::read_chunk_4),
                BodyChunk::normal(5, Self::read_chunk_5),
                BodyChunk::normal(6, Self::read_chunk_6),
                BodyChunk::skippable(8, Self::read_chunk_8),
            ]
            .into_iter()
        }
    }

    impl BlockInfoClip {
        fn read_chunk_2<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let _asymmetrical_clip_id = r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _is_full_free_clip = r.bool()?;
            let _is_exclusive_free_clip = r.bool()?;

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _clip_type = r.u32()?;

            Ok(())
        }

        fn read_chunk_6<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 4 {
                return Err(Error::chunk_version(version));
            }

            let _can_be_deleted_by_full_free_clip = r.bool()?;
            let _top_bottom_multi_dir = r.u32()?;
            r.u8()?;
            r.u8()?;
            r.u8()?;

            Ok(())
        }

        fn read_chunk_8<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let _clip_group_id = r.id_or_null()?;
            let _symmetrical_clip_group_id = r.id_or_null()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
