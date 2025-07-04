use std::ops::{Deref, DerefMut};

use crate::{ClassId, class::game::block_info_variant::BlockInfoVariant};

/// Block info variant ground.
#[derive(Default)]
pub struct BlockInfoVariantGround {
    parent: BlockInfoVariant,
}

impl ClassId for BlockInfoVariantGround {
    const CLASS_ID: u32 = 0x0315c000;
}

impl Deref for BlockInfoVariantGround {
    type Target = BlockInfoVariant;

    fn deref(&self) -> &BlockInfoVariant {
        &self.parent
    }
}

impl DerefMut for BlockInfoVariantGround {
    fn deref_mut(&mut self) -> &mut BlockInfoVariant {
        &mut self.parent
    }
}

mod read {
    use crate::{
        class::game::block_info_variant_ground::BlockInfoVariantGround,
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for BlockInfoVariantGround {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for BlockInfoVariantGround {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            []
        }
    }
}
