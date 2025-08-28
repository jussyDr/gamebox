pub mod media;

mod anchored_object;
use std::sync::Arc;

pub use anchored_object::AnchoredObject;

pub mod block;
pub use block::Block;

mod block_skin;
pub use block_skin::BlockSkin;

pub mod challenge;
pub use challenge::Challenge;

mod challenge_parameters;
pub use challenge_parameters::ChallengeParameters;

mod collector_list;
pub use collector_list::CollectorList;

mod ghost;
pub use ghost::Ghost;

mod zone_genealogy;
pub use zone_genealogy::ZoneGenealogy;

use crate::read::{Error, HeaderReader, ReadEnum, Reader, Result};

/// Cardinal direction.
pub enum Direction {
    /// North.
    North,
    /// East.
    East,
    /// South.
    South,
    /// West.
    West,
}

impl ReadEnum for Direction {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::North),
            1 => Ok(Self::East),
            2 => Ok(Self::South),
            3 => Ok(Self::West),
            _ => Err(Error::Internal(
                "unknown variant index for enum Direction".into(),
            )),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum ElemColor {
    #[default]
    Default,
    White,
    Green,
    Blue,
    Red,
    Black,
}

impl ReadEnum for ElemColor {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::Default),
            1 => Ok(Self::White),
            2 => Ok(Self::Green),
            3 => Ok(Self::Blue),
            4 => Ok(Self::Red),
            5 => Ok(Self::Black),
            _ => Err(Error::Internal(
                "unknown variant index for enum ElemColor".into(),
            )),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum PhaseOffset {
    #[default]
    None,
    One8th,
    Two8th,
    Three8th,
    Four8th,
    Five8th,
    Six8th,
    Seven8th,
}

impl ReadEnum for PhaseOffset {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::None),
            1 => Ok(Self::One8th),
            2 => Ok(Self::Two8th),
            3 => Ok(Self::Three8th),
            4 => Ok(Self::Four8th),
            5 => Ok(Self::Five8th),
            6 => Ok(Self::Six8th),
            7 => Ok(Self::Seven8th),
            _ => Err(Error::Internal(
                "unknown variant index for enum PhaseOffset".into(),
            )),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum LightmapQuality {
    #[default]
    Normal,
    High,
    VeryHigh,
    Highest,
    Low,
    VeryLow,
    Lowest,
}

impl ReadEnum for LightmapQuality {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::Normal),
            1 => Ok(Self::High),
            2 => Ok(Self::VeryHigh),
            3 => Ok(Self::Highest),
            4 => Ok(Self::Low),
            5 => Ok(Self::VeryLow),
            6 => Ok(Self::Lowest),
            _ => Err(Error::Internal(
                "unknown variant index for enum LightmapQuality".into(),
            )),
        }
    }
}

/// Reference to a file.
pub enum FileRef {
    Internal {
        path: String,
    },
    External {
        path: String,
        url: String,
        checksum: [u8; 32],
    },
}

impl FileRef {
    fn read(r: &mut impl Reader) -> Result<Option<Self>> {
        let version = r.u8()?;

        if version != 3 {
            return Err(Error::Internal("unknown file reference version".into()));
        }

        let checksum = r.array_u8::<32>()?;
        let path = r.string()?;
        let url = r.string()?;

        if path.is_empty() {
            return Ok(None);
        }

        if url.is_empty() {
            return Ok(Some(FileRef::Internal { path }));
        }

        Ok(Some(FileRef::External {
            path,
            url,
            checksum,
        }))
    }
}

struct Ident {
    id: Option<Arc<str>>,
    collection: Option<Arc<str>>,
    author: Option<Arc<str>>,
}

impl Ident {
    fn read(r: &mut impl HeaderReader) -> Result<Self> {
        let id = r.string_ref()?;
        let collection = r.string_ref()?;
        let author = r.string_ref()?;

        Ok(Self {
            id,
            collection,
            author,
        })
    }
}
