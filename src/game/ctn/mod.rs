//! Game ctn engine.

pub mod anchored_object;
pub mod auto_terrain;
pub mod block;
pub mod block_info;
pub mod block_info_classic;
pub mod block_info_clip;
pub mod block_info_clip_horizontal;
pub mod block_info_clip_vertical;
pub mod block_info_flat;
pub mod block_info_mobil;
pub mod block_info_variant;
pub mod block_info_variant_air;
pub mod block_info_variant_ground;
pub mod block_skin;
pub mod block_unit_info;
pub mod challenge;
pub mod challenge_parameters;
pub mod collection;
pub mod collector;
pub mod collector_list;
pub mod decoration;
pub mod decoration_mood;
pub mod ghost;
pub mod media_block;
pub mod media_block_camera_custom;
pub mod media_block_camera_effect_shake;
pub mod media_block_camera_game;
pub mod media_block_camera_path;
pub mod media_block_color_grading;
pub mod media_block_dirty_lens;
pub mod media_block_dof;
pub mod media_block_entity;
pub mod media_block_fog;
pub mod media_block_fx_colors;
pub mod media_block_image;
pub mod media_block_interface;
pub mod media_block_mania_link;
pub mod media_block_sound;
pub mod media_block_text;
pub mod media_block_tone_mapping;
pub mod media_block_trails;
pub mod media_block_transition_fade;
pub mod media_block_triangles;
pub mod media_block_triangles_2d;
pub mod media_block_triangles_3d;
pub mod media_clip;
pub mod media_clip_group;
pub mod media_track;
pub mod zone_genealogy;

#[doc(inline)]
pub use anchored_object::AnchoredObject;
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
pub use block_info_clip_horizontal::BlockInfoClipHorizontal;
#[doc(inline)]
pub use block_info_clip_vertical::BlockInfoClipVertical;
#[doc(inline)]
pub use block_info_flat::BlockInfoFlat;
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
pub use collection::Collection;
#[doc(inline)]
pub use collector::Collector;
#[doc(inline)]
pub use collector_list::CollectorList;
#[doc(inline)]
pub use decoration::Decoration;
#[doc(inline)]
pub use decoration_mood::DecorationMood;
#[doc(inline)]
pub use ghost::Ghost;
#[doc(inline)]
pub use media_block::MediaBlock;
#[doc(inline)]
pub use media_block_camera_custom::MediaBlockCameraCustom;
#[doc(inline)]
pub use media_block_camera_effect_shake::MediaBlockCameraEffectShake;
#[doc(inline)]
pub use media_block_camera_path::MediaBlockCameraPath;
#[doc(inline)]
pub use media_block_color_grading::MediaBlockColorGrading;
#[doc(inline)]
pub use media_block_dirty_lens::MediaBlockDirtyLens;
#[doc(inline)]
pub use media_block_fog::MediaBlockFog;
#[doc(inline)]
pub use media_block_fx_colors::MediaBlockFxColors;
#[doc(inline)]
pub use media_block_mania_link::MediaBlockManialink;
#[doc(inline)]
pub use media_block_sound::MediaBlockSound;
#[doc(inline)]
pub use media_block_tone_mapping::MediaBlockToneMapping;
#[doc(inline)]
pub use media_block_transition_fade::MediaBlockTransitionFade;
#[doc(inline)]
pub use media_block_triangles_2d::MediaBlockTriangles2D;
#[doc(inline)]
pub use media_block_triangles_3d::MediaBlockTriangles3D;
#[doc(inline)]
pub use media_clip::MediaClip;
#[doc(inline)]
pub use media_clip_group::MediaClipGroup;
#[doc(inline)]
pub use media_track::MediaTrack;
#[doc(inline)]
pub use zone_genealogy::ZoneGenealogy;

/// Cardinal direction.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
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

/// Element color.
#[derive(Clone, Copy, Default, Debug)]
pub enum ElemColor {
    /// Default color.
    #[default]
    Default,
    /// White.
    White,
    /// Green.
    Green,
    /// Blue.
    Blue,
    /// Red.
    Red,
    /// Black.
    Black,
}

impl TryFrom<u8> for ElemColor {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::White),
            2 => Ok(Self::Green),
            3 => Ok(Self::Blue),
            4 => Ok(Self::Red),
            5 => Ok(Self::Black),
            _ => Err(()),
        }
    }
}

/// Lightmap quality
#[derive(Clone, Copy, Default, Debug)]
pub enum LightmapQuality {
    /// Normal quality.
    #[default]
    Normal,
    /// High quality.
    High,
    /// Very high quality.
    VeryHigh,
    /// Highest quality.
    Highest,
    /// Low quality.
    Low,
    /// Very low quality.
    VeryLow,
    /// Lowest quality.
    Lowest,
}

impl TryFrom<u8> for LightmapQuality {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::High),
            2 => Ok(Self::VeryHigh),
            3 => Ok(Self::Highest),
            4 => Ok(Self::Low),
            5 => Ok(Self::VeryLow),
            6 => Ok(Self::Lowest),
            _ => Err(()),
        }
    }
}
