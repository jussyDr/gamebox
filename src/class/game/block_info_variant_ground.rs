//! Block info variant ground.

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
        class::game::{
            auto_terrain::AutoTerrain, block_info_variant_ground::BlockInfoVariantGround,
        },
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
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
            [BodyChunk::new(1, Self::read_chunk_1)]
        }
    }

    impl BlockInfoVariantGround {
        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(error_unknown_chunk_version(version));
            }

            let _auto_terrains = r.list_with_version(|r| r.internal_node_ref::<AutoTerrain>())?;
            let _auto_terrain_height_offset = r.u32()?;
            let _auto_terrain_place_type = r.u32()?;

            Ok(())
        }
    }
}
