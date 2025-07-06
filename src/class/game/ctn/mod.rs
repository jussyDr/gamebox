//! CTN.

pub mod anchored_object;
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
pub mod ghost;
pub mod media_block_camera_custom;
pub mod media_block_fog;
pub mod media_block_image;
pub mod media_clip;
pub mod media_clip_group;
pub mod media_track;
pub mod zone_genealogy;

use crate::read::{Error, error_unknown_version, reader::Reader};

/// Reference to a file.
pub enum FileRef {
    /// Reference to an internal game file.
    Internal {
        /// Path.
        path: String,
    },
    /// Reference to an external file.
    External {
        /// Checksum.
        checksum: [u8; 32],
        /// Path.
        path: String,
        /// Locator URL.
        locator_url: String,
    },
}

fn read_file_ref(r: &mut impl Reader) -> Result<Option<FileRef>, Error> {
    let version = r.u8()?;

    if version != 3 {
        return Err(error_unknown_version("file reference", version as u32));
    }

    let checksum: [u8; 32] = r.byte_array()?;
    let path = r.string()?;
    let locator_url = r.string()?;

    if checksum.iter().all(|&byte| byte == 0) && path.is_empty() && locator_url.is_empty() {
        return Ok(None);
    }

    if checksum[0] == 2 && checksum[1..].iter().all(|&byte| byte == 0) && locator_url.is_empty() {
        return Ok(Some(FileRef::Internal { path }));
    }

    Ok(Some(FileRef::External {
        checksum,
        path,
        locator_url,
    }))
}
