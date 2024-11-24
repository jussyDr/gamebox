use std::ops::Deref;

use crate::Class;

use super::block_info_variant::BlockInfoVariant;

/// A block info variant air.
#[derive(Default)]
pub struct BlockInfoVariantAir {
    parent: BlockInfoVariant,
}

impl Class for BlockInfoVariantAir {
    const CLASS_ID: u32 = 0x0315d000;
}

impl Deref for BlockInfoVariantAir {
    type Target = BlockInfoVariant;

    fn deref(&self) -> &BlockInfoVariant {
        &self.parent
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::BlockInfoVariantAir;

    impl ReadBody for BlockInfoVariantAir {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for BlockInfoVariantAir {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }
}
