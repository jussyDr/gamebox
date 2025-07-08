//! Block info.

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{
    ClassId,
    class::game::ctn::{
        block_info_variant_air::BlockInfoVariantAir,
        block_info_variant_ground::BlockInfoVariantGround, collector::Collector,
    },
};

/// Block info.
#[derive(Default)]
pub struct BlockInfo {
    parent: Collector,
    variant_base_ground: BlockInfoVariantGround,
    variant_base_air: BlockInfoVariantAir,
    additional_variants_ground: Vec<Arc<BlockInfoVariantGround>>,
    additional_variants_air: Vec<Arc<BlockInfoVariantAir>>,
}

impl BlockInfo {
    /// Variant base ground.
    pub fn variant_base_ground(&self) -> &BlockInfoVariantGround {
        &self.variant_base_ground
    }

    /// Variant base air.
    pub fn variant_base_air(&self) -> &BlockInfoVariantAir {
        &self.variant_base_air
    }

    /// Additional variants ground.
    pub fn additional_variants_ground(&self) -> &Vec<Arc<BlockInfoVariantGround>> {
        &self.additional_variants_ground
    }

    /// Additional variants air.
    pub fn additional_variants_air(&self) -> &Vec<Arc<BlockInfoVariantAir>> {
        &self.additional_variants_air
    }
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
    use std::sync::Arc;

    use crate::{
        Delme,
        class::{
            game::{
                ctn::{
                    block_info::BlockInfo, block_info_variant_air::BlockInfoVariantAir,
                    block_info_variant_ground::BlockInfoVariantGround,
                },
                podium_info::PodiumInfo,
            },
            plug::{
                game_skin_and_folder::GameSkinAndFolder, media_clip_list::MediaClipList,
                sound::Sound,
            },
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
                BodyChunk::new(38, Self::read_chunk_38),
                BodyChunk::new(39, Self::read_chunk_39),
                BodyChunk::new(40, Self::read_chunk_40),
                BodyChunk::new(41, Self::read_chunk_41),
                BodyChunk::new(42, Self::read_chunk_42),
                BodyChunk::new(43, Self::read_chunk_43),
                BodyChunk::new(44, Self::read_chunk_44),
                BodyChunk::new(47, Self::read_chunk_47),
                BodyChunk::new(49, Self::read_chunk_49),
            ]
        }
    }

    impl BlockInfo {
        fn read_chunk_15(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _no_respawn = r.bool32()?;

            Ok(())
        }

        fn read_chunk_19(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _icon_use_auto_ground = r.bool32()?;

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
            let _char_phy_special_property = r.u32()?;
            let _podium_info = r.u32()?;
            let _intro_info = r.internal_node_ref_or_null::<MediaClipList>()?;
            let _char_phy_special_property_customizable = r.bool32()?;

            if r.bool32()? {
                r.string()?;
                r.string()?;
            }

            Ok(())
        }

        fn read_chunk_35(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            self.variant_base_ground = read_node_from_body::<BlockInfoVariantGround>(r)?;
            self.variant_base_air = read_node_from_body::<BlockInfoVariantAir>(r)?;

            Ok(())
        }

        fn read_chunk_38(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _waypoint_type = r.u32()?;

            Ok(())
        }

        fn read_chunk_39(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            self.additional_variants_ground =
                r.list_with_version(|r| r.internal_node_ref::<BlockInfoVariantGround>())?;

            Ok(())
        }

        fn read_chunk_40(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _symmetrical_block_info_id: Option<Arc<str>> = r.id()?;
            let _dir = r.u32()?;

            Ok(())
        }

        fn read_chunk_41(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _fog_volume_box = r.external_node_ref_or_null::<Delme>()?;

            Ok(())
        }

        fn read_chunk_42(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(error_unknown_chunk_version(version));
            }

            let sound_1 = r.external_node_ref_or_null::<Sound>()?;
            let sound_2 = r.external_node_ref_or_null::<Sound>()?;

            if sound_1.is_some() {
                let _sound_1_loc = r.iso4()?;
            }

            if sound_2.is_some() {
                let _sound_2_loc = r.iso4()?;
            }

            Ok(())
        }

        fn read_chunk_43(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            let _base_type = r.u32()?;

            Ok(())
        }

        fn read_chunk_44(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            self.additional_variants_air =
                r.list_with_version(|r| r.internal_node_ref::<BlockInfoVariantAir>())?;

            Ok(())
        }

        fn read_chunk_47(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            let _is_pillar = r.bool8()?;
            let _pillar_shape_multi_dir = r.u8()?;
            r.u8()?;

            Ok(())
        }

        fn read_chunk_49(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            r.u32()?;
            let _material_modifier = r.external_node_ref_or_null::<GameSkinAndFolder>()?;
            let _material_modifier_2 = r.external_node_ref_or_null::<GameSkinAndFolder>()?;

            Ok(())
        }
    }
}
