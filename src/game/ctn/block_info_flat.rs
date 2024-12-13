//! Block info flat.

use std::ops::Deref;

use crate::Class;

use super::BlockInfo;

/// Block info flat.
#[derive(Default)]
pub struct BlockInfoFlat {
    parent: BlockInfo,
}

impl Class for BlockInfoFlat {
    const CLASS_ID: u32 = 0x0304f000;
}

impl Deref for BlockInfoFlat {
    type Target = BlockInfo;

    fn deref(&self) -> &BlockInfo {
        &self.parent
    }
}

impl BlockInfoFlat {
    pub fn into_parent(self) -> BlockInfo {
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

    use super::BlockInfoFlat;

    impl Readable for BlockInfoFlat {}

    impl readable::Sealed for BlockInfoFlat {}

    impl HeaderChunks for BlockInfoFlat {
        fn parent(&mut self) -> Option<&mut impl HeaderChunks> {
            Some(&mut self.parent)
        }

        fn header_chunks<R, I, N>() -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

    impl ReadBody for BlockInfoFlat {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for BlockInfoFlat {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }
}
