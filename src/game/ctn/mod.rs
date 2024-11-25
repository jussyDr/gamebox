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
