//! Game ctn engine.

pub mod auto_terrain;
pub mod block;
pub mod block_info;
pub mod block_info_classic;
pub mod block_info_clip;
pub mod block_info_mobil;
pub mod block_info_variant;
pub mod block_info_variant_air;
pub mod block_info_variant_ground;
pub mod block_skin;
pub mod block_unit_info;
pub mod challenge;
pub mod challenge_parameters;
pub mod collector;
pub mod collector_list;
pub mod decoration;
pub mod ghost;
pub mod media_clip;
pub mod media_clip_group;
pub mod zone_genealogy;

#[doc(inline)]
pub use auto_terrain::AutoTerrain;
#[doc(inline)]
pub use block::Block;
#[doc(inline)]
pub use block_info::BlockInfo;
#[doc(inline)]
pub use block_info_classic::BlockInfoClassic;
#[doc(inline)]
pub use block_info_clip::BlockInfoClip;
#[doc(inline)]
pub use block_info_mobil::BlockInfoMobil;
#[doc(inline)]
pub use block_info_variant::BlockInfoVariant;
#[doc(inline)]
pub use block_info_variant_air::BlockInfoVariantAir;
#[doc(inline)]
pub use block_info_variant_ground::BlockInfoVariantGround;
#[doc(inline)]
pub use block_skin::BlockSkin;
#[doc(inline)]
pub use block_unit_info::BlockUnitInfo;
#[doc(inline)]
pub use challenge::Challenge;
#[doc(inline)]
pub use challenge_parameters::ChallengeParameters;
#[doc(inline)]
pub use collector::Collector;
#[doc(inline)]
pub use collector_list::CollectorList;
#[doc(inline)]
pub use decoration::Decoration;
#[doc(inline)]
pub use ghost::Ghost;
#[doc(inline)]
pub use media_clip::MediaClip;
#[doc(inline)]
pub use media_clip_group::MediaClipGroup;
#[doc(inline)]
pub use zone_genealogy::ZoneGenealogy;

/// Cardinal direction.
#[derive(Clone, Copy, Default)]
pub enum Direction {
    /// North.
    #[default]
    North,
    /// East.
    East,
    /// South.
    South,
    /// West.
    West,
}

impl TryFrom<u32> for Direction {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::North),
            1 => Ok(Self::East),
            2 => Ok(Self::South),
            3 => Ok(Self::West),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        (value as u32).try_into()
    }
}
