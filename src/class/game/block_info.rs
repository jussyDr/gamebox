use std::ops::{Deref, DerefMut};

use crate::{ClassId, class::game_data::collector::Collector};

/// Block info.
#[derive(Default)]
pub struct BlockInfo {
    parent: Collector,
}

impl ClassId for BlockInfo {
    const CLASS_ID: u32 = 0x0304e000;
}

impl Deref for BlockInfo {
    type Target = Collector;

    fn deref(&self) -> &Collector {
        &self.parent
    }
}

impl DerefMut for BlockInfo {
    fn deref_mut(&mut self) -> &mut Collector {
        &mut self.parent
    }
}

mod read {
    use crate::{
        class::game::{
            block_info::BlockInfo, block_info_variant_air::BlockInfoVariantAir,
            block_info_variant_ground::BlockInfoVariantGround,
        },
        read::{
            BodyChunk, BodyChunks, Error, error_unknown_chunk_version, read_node_from_body,
            reader::BodyReader,
        },
    };

    impl BodyChunks for BlockInfo {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(15, Self::read_chunk_15),
                BodyChunk::new(19, Self::read_chunk_19),
                BodyChunk::new(23, Self::read_chunk_23),
                BodyChunk::new(32, Self::read_chunk_32),
                BodyChunk::new(35, Self::read_chunk_35),
            ]
        }
    }

    impl BlockInfo {
        fn read_chunk_15(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let no_respawn = r.bool32()?;

            Ok(())
        }

        fn read_chunk_19(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let icon_use_auto_ground = r.bool32()?;

            Ok(())
        }

        fn read_chunk_23(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.bool32()?;

            Ok(())
        }

        fn read_chunk_32(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 8 {
                return Err(error_unknown_chunk_version(version));
            }

            let char_phy_special_property = r.u32()?;
            let podium_info = r.u32()?;
            let intro_info = r.u32()?;
            let char_phy_special_property_customizable = r.bool32()?;

            if r.bool32()? {
                todo!()
            }

            Ok(())
        }

        fn read_chunk_35(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let variant_base_ground = read_node_from_body::<BlockInfoVariantGround>(r)?;
            let variant_base_air = read_node_from_body::<BlockInfoVariantAir>(r)?;

            Ok(())
        }
    }
}
