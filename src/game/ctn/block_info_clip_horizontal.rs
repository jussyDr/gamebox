//! Block info clip horizontal.

use std::ops::Deref;

use crate::Class;

use super::BlockInfoClip;

/// Block info clip horizontal.
#[derive(Default)]
pub struct BlockInfoClipHorizontal {
    parent: BlockInfoClip,
}

impl Class for BlockInfoClipHorizontal {
    const CLASS_ID: u32 = 0x0335b000;
}

impl Deref for BlockInfoClipHorizontal {
    type Target = BlockInfoClip;

    fn deref(&self) -> &BlockInfoClip {
        &self.parent
    }
}

impl BlockInfoClipHorizontal {
    pub fn into_parent(self) -> BlockInfoClip {
        self.parent
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        readable,
        reader::{IdStateMut, NodeStateMut, Reader},
        Error, Readable,
    };

    use self::readable::{
        read_body_chunks, BodyChunk, BodyChunks, HeaderChunk, HeaderChunks, ReadBody,
    };

    use super::BlockInfoClipHorizontal;

    impl Readable for BlockInfoClipHorizontal {}

    impl readable::Sealed for BlockInfoClipHorizontal {}

    impl HeaderChunks for BlockInfoClipHorizontal {
        fn parent(&mut self) -> Option<&mut impl HeaderChunks> {
            Some(&mut self.parent)
        }

        fn header_chunks<R: Read, I: IdStateMut, N>(
        ) -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

    impl ReadBody for BlockInfoClipHorizontal {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for BlockInfoClipHorizontal {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>>
        {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl BlockInfoClipHorizontal {
        fn read_chunk_0<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.id_or_null()?;

            Ok(())
        }
    }
}
