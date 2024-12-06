//! Media block.

use super::{
    MediaBlockCameraCustom, MediaBlockFog, MediaBlockManialink, MediaBlockTransitionFade,
    MediaBlockTriangles3D,
};

/// A media block.
pub enum MediaBlock {
    Triangles3D(MediaBlockTriangles3D),
    CameraCustom(MediaBlockCameraCustom),
    TransitionFade(MediaBlockTransitionFade),
    Manialink(MediaBlockManialink),
    Fog(MediaBlockFog),
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::{
            media_block_mania_link::MediaBlockManialink, MediaBlockCameraCustom, MediaBlockFog,
            MediaBlockTransitionFade, MediaBlockTriangles3D,
        },
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, ErrorKind,
        },
    };

    use super::MediaBlock;

    impl MediaBlock {
        pub(crate) fn read(
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
            class_id: u32,
        ) -> Result<Self, Error> {
            match class_id {
                0x0304c000 => {
                    let mut media_block_triangles_3d = MediaBlockTriangles3D::default();
                    read_body_chunks(&mut media_block_triangles_3d, r)?;

                    Ok(Self::Triangles3D(media_block_triangles_3d))
                }
                0x030a2000 => {
                    let mut media_block_camera_custom = MediaBlockCameraCustom::default();
                    read_body_chunks(&mut media_block_camera_custom, r)?;

                    Ok(Self::CameraCustom(media_block_camera_custom))
                }
                0x030ab000 => {
                    let mut media_block_transition_fade = MediaBlockTransitionFade::default();
                    read_body_chunks(&mut media_block_transition_fade, r)?;

                    Ok(Self::TransitionFade(media_block_transition_fade))
                }
                0x0312a000 => {
                    let mut media_block_manialink = MediaBlockManialink::default();
                    read_body_chunks(&mut media_block_manialink, r)?;

                    Ok(Self::Manialink(media_block_manialink))
                }
                0x03199000 => {
                    let mut media_block_fog = MediaBlockFog::default();
                    read_body_chunks(&mut media_block_fog, r)?;

                    Ok(Self::Fog(media_block_fog))
                }
                _ => {
                    return Err(Error::new(ErrorKind::Unsupported(format!(
                        "{class_id:08x?}"
                    ))))
                }
            }
        }
    }
}
