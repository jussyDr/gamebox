//! Media block.

use super::{
    media_block_interface::MediaBlockInterface, media_block_text::MediaBlockText,
    MediaBlockCameraCustom, MediaBlockFog, MediaBlockManialink, MediaBlockSound,
    MediaBlockTransitionFade, MediaBlockTriangles2D, MediaBlockTriangles3D,
};

/// Media block.
pub enum MediaBlock {
    /// 2D triangles.
    Triangles2D(MediaBlockTriangles2D),
    /// 3D triangles.
    Triangles3D(MediaBlockTriangles3D),
    /// Custom camera.
    CameraCustom(MediaBlockCameraCustom),
    /// Sound.
    Sound(MediaBlockSound),
    /// Text.
    Text(MediaBlockText),
    /// Transition fade.
    TransitionFade(MediaBlockTransitionFade),
    /// Manialink.
    Manialink(MediaBlockManialink),
    /// Interface
    Interface(MediaBlockInterface),
    /// Fog.
    Fog(MediaBlockFog),
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::{
            media_block_interface::MediaBlockInterface,
            media_block_mania_link::MediaBlockManialink, media_block_text::MediaBlockText,
            MediaBlockCameraCustom, MediaBlockFog, MediaBlockSound, MediaBlockTransitionFade,
            MediaBlockTriangles2D, MediaBlockTriangles3D,
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
                0x0304b000 => {
                    let mut media_block_triangles_2d = MediaBlockTriangles2D::default();
                    read_body_chunks(&mut media_block_triangles_2d, r)?;

                    Ok(Self::Triangles2D(media_block_triangles_2d))
                }
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
                0x030a7000 => {
                    let mut sound = MediaBlockSound::default();
                    read_body_chunks(&mut sound, r)?;

                    Ok(Self::Sound(sound))
                }
                0x030a8000 => {
                    let mut media_block_text = MediaBlockText::default();
                    read_body_chunks(&mut media_block_text, r)?;

                    Ok(Self::Text(media_block_text))
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
                0x03195000 => {
                    let mut media_block_interface = MediaBlockInterface::default();
                    read_body_chunks(&mut media_block_interface, r)?;

                    Ok(Self::Interface(media_block_interface))
                }
                0x03199000 => {
                    let mut media_block_fog = MediaBlockFog::default();
                    read_body_chunks(&mut media_block_fog, r)?;

                    Ok(Self::Fog(media_block_fog))
                }
                _ => Err(Error::new(ErrorKind::Unsupported(format!(
                    "{class_id:08x?}"
                )))),
            }
        }
    }
}
