//! Block info variant air.

use std::ops::{Deref, DerefMut};

use crate::{ClassId, class::game::ctn::block_info_variant::BlockInfoVariant};

/// Block info variant air.
#[derive(Default)]
pub struct BlockInfoVariantAir {
    parent: BlockInfoVariant,
}

impl ClassId for BlockInfoVariantAir {
    const CLASS_ID: u32 = 0x0315d000;
}

impl Deref for BlockInfoVariantAir {
    type Target = BlockInfoVariant;

    fn deref(&self) -> &BlockInfoVariant {
        &self.parent
    }
}

impl DerefMut for BlockInfoVariantAir {
    fn deref_mut(&mut self) -> &mut BlockInfoVariant {
        &mut self.parent
    }
}

mod read {
    use crate::{
        class::game::ctn::block_info_variant_air::BlockInfoVariantAir,
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for BlockInfoVariantAir {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for BlockInfoVariantAir {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            []
        }
    }
}
