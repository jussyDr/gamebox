//! Block info variant ground.

use std::{ops::Deref, sync::Arc};

use crate::Class;

use super::{block_info_variant::BlockInfoVariant, AutoTerrain};

/// A block info variant ground.
#[derive(Default)]
pub struct BlockInfoVariantGround {
    parent: BlockInfoVariant,
    auto_terrains: Vec<Arc<AutoTerrain>>,
}

impl Class for BlockInfoVariantGround {
    const CLASS_ID: u32 = 0x0315c000;
}

impl Deref for BlockInfoVariantGround {
    type Target = BlockInfoVariant;

    fn deref(&self) -> &BlockInfoVariant {
        &self.parent
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::auto_terrain::AutoTerrain,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::BlockInfoVariantGround;

    impl ReadBody for BlockInfoVariantGround {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for BlockInfoVariantGround {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(1, Self::read_chunk_1)].into_iter()
        }
    }

    impl BlockInfoVariantGround {
        fn read_chunk_1(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            self.auto_terrains = r.list_with_version(|r| r.internal_node_ref::<AutoTerrain>())?;
            let _auto_terrain_height_offset = r.u32()?;
            let _auto_terrain_place_type = r.u32()?;

            Ok(())
        }
    }
}
