//! Block info classic.

use std::ops::Deref;

use crate::Class;

use super::block_info::BlockInfo;

/// A block info classic
#[derive(Default)]
pub struct BlockInfoClassic {
    parent: BlockInfo,
}

impl Class for BlockInfoClassic {
    const CLASS_ID: u32 = 0x03051000;
}

impl Deref for BlockInfoClassic {
    type Target = BlockInfo;

    fn deref(&self) -> &BlockInfo {
        &self.parent
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        readable::Sealed,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody, Readable,
    };

    use super::BlockInfoClassic;

    impl Readable for BlockInfoClassic {}

    impl Sealed for BlockInfoClassic {}

    impl ReadBody for BlockInfoClassic {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for BlockInfoClassic {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }
}
