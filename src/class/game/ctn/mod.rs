mod anchored_object;
mod block;
mod block_skin;
mod challenge;
mod challenge_parameters;
mod collector_list;
mod ghost;
mod media_block;
mod media_block_camera_custom;
mod media_block_camera_game;
mod media_block_color_grading;
mod media_block_dof;
mod media_block_entity;
mod media_block_fog;
mod media_block_image;
mod media_block_text;
mod media_block_time;
mod media_block_transition_fade;
mod media_clip;
mod media_clip_group;
mod media_track;
mod zone_genealogy;

pub use anchored_object::AnchoredObject;
pub use block::Block;
pub use block_skin::BlockSkin;
pub use challenge::Challenge;
pub use challenge_parameters::ChallengeParameters;
pub use collector_list::CollectorList;
pub use ghost::Ghost;
pub use media_block::MediaBlock;
pub use media_block_camera_custom::MediaBlockCameraCustom;
pub use media_block_camera_game::MediaBlockCameraGame;
pub use media_block_color_grading::MediaBlockColorGrading;
pub use media_block_dof::MediaBlockDOF;
pub use media_block_entity::MediaBlockEntity;
pub use media_block_fog::MediaBlockFog;
pub use media_block_image::MediaBlockImage;
pub use media_block_text::MediaBlockText;
pub use media_block_time::MediaBlockTime;
pub use media_block_transition_fade::MediaBlockTransitionFade;
pub use media_clip::MediaClip;
pub use media_clip_group::MediaClipGroup;
pub use media_track::MediaTrack;
pub use zone_genealogy::ZoneGenealogy;

use crate::read::{BodyReader, Error};

pub enum FileRef<'a> {
    Internal {
        path: &'a str,
    },
    External {
        checksum: [u8; 32],
        path: &'a str,
        locator_url: &'a str,
    },
}

impl<'a> FileRef<'a> {
    pub fn read(r: &mut BodyReader<'a, '_>) -> Result<Option<Self>, Error> {
        let version = r.u8()?;

        if version != 3 {
            return Err(Error::new(format!(
                "unknown file reference version: {version}"
            )));
        }

        let checksum: [u8; 32] = r.u8_array()?;
        let path = r.string()?;
        let locator_url = r.string()?;

        if checksum.iter().all(|&byte| byte == 0) && path.is_empty() && locator_url.is_empty() {
            return Ok(None);
        }

        if checksum[0] == 2 && checksum[1..].iter().all(|&byte| byte == 0) && locator_url.is_empty()
        {
            return Ok(Some(FileRef::Internal { path }));
        }

        Ok(Some(FileRef::External {
            checksum,
            path,
            locator_url,
        }))
    }
}
