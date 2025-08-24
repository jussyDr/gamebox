mod anchored_object;
pub use anchored_object::AnchoredObject;

mod block;
pub use block::Block;

mod block_skin;
pub use block_skin::BlockSkin;

mod challenge;
pub use challenge::Challenge;

mod challenge_parameters;
pub use challenge_parameters::ChallengeParameters;

mod collector_list;
pub use collector_list::CollectorList;

mod ghost;
pub use ghost::Ghost;

mod media_block;
pub use media_block::MediaBlock;

mod media_block_camera_custom;
pub use media_block_camera_custom::MediaBlockCameraCustom;

mod media_block_camera_game;
pub use media_block_camera_game::MediaBlockCameraGame;

mod media_block_color_grading;
pub use media_block_color_grading::MediaBlockColorGrading;

mod media_block_dof;
pub use media_block_dof::MediaBlockDOF;

mod media_block_entity;
pub use media_block_entity::MediaBlockEntity;

mod media_block_fog;
pub use media_block_fog::MediaBlockFog;

mod media_block_image;
pub use media_block_image::MediaBlockImage;

mod media_block_text;
pub use media_block_text::MediaBlockText;

mod media_block_time;
pub use media_block_time::MediaBlockTime;

mod media_block_transition_fade;
pub use media_block_transition_fade::MediaBlockTransitionFade;

mod media_clip;
pub use media_clip::MediaClip;

mod media_clip_group;
pub use media_clip_group::MediaClipGroup;

mod media_track;
pub use media_track::MediaTrack;

mod zone_genealogy;
pub use zone_genealogy::ZoneGenealogy;

use crate::read::{Error, ReadEnum, Reader, Result};

enum Direction {
    North,
    East,
    South,
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

enum FileRef {
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
