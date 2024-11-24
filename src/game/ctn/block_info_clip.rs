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

mod read {
    use std::io::Read;

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::BlockInfoClip;

    impl ReadBody for BlockInfoClip {
        fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
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

        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }
}
