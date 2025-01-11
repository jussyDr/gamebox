//! Media block.

use super::{
    media_block_camera_game::MediaBlockCameraGame, media_block_dof::MediaBlockDof,
    media_block_entity::MediaBlockEntity, media_block_image::MediaBlockImage,
    media_block_interface::MediaBlockInterface, media_block_text::MediaBlockText,
    media_block_trails::MediaBlockTrails, MediaBlockCameraCustom, MediaBlockCameraEffectShake,
    MediaBlockCameraPath, MediaBlockColorGrading, MediaBlockDirtyLens, MediaBlockFog,
    MediaBlockFxColors, MediaBlockManialink, MediaBlockSound, MediaBlockToneMapping,
    MediaBlockTransitionFade, MediaBlockTriangles2D, MediaBlockTriangles3D,
};

/// Media block.
#[derive(PartialEq, Eq, Hash)]
pub enum MediaBlock {
    /// 2D triangles.
    Triangles2D(MediaBlockTriangles2D),
    /// 3D triangles.
    Triangles3D(MediaBlockTriangles3D),
    /// Fx colors.
    FxColors(MediaBlockFxColors),
    /// Camera game.
    CameraGame(MediaBlockCameraGame),
    /// Camera path.
    CameraPath(MediaBlockCameraPath),
    /// Custom camera.
    CameraCustom(MediaBlockCameraCustom),
    /// Camera effect shake.
    CameraEffectShake(MediaBlockCameraEffectShake),
    /// Image.
    Image(MediaBlockImage),
    /// Sound.
    Sound(MediaBlockSound),
    /// Text.
    Text(MediaBlockText),
    /// Trails
    Trails(MediaBlockTrails),
    /// Transition fade.
    TransitionFade(MediaBlockTransitionFade),
    /// DOF.
    Dof(MediaBlockDof),
    /// Tone mapping.
    ToneMapping(MediaBlockToneMapping),
    /// Manialink.
    Manialink(MediaBlockManialink),
    /// Dirty lens.
    DirtyLens(MediaBlockDirtyLens),
    /// Color grading.
    ColorGrading(MediaBlockColorGrading),
    /// Interface
    Interface(MediaBlockInterface),
    /// Fog.
    Fog(MediaBlockFog),
    /// Entity.
    Entity(MediaBlockEntity),
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::{
            media_block_camera_game::MediaBlockCameraGame,
            media_block_color_grading::MediaBlockColorGrading, media_block_dof::MediaBlockDof,
            media_block_entity::MediaBlockEntity, media_block_image::MediaBlockImage,
            media_block_interface::MediaBlockInterface,
            media_block_mania_link::MediaBlockManialink, media_block_text::MediaBlockText,
            media_block_trails::MediaBlockTrails, MediaBlockCameraCustom,
            MediaBlockCameraEffectShake, MediaBlockCameraPath, MediaBlockDirtyLens, MediaBlockFog,
            MediaBlockFxColors, MediaBlockSound, MediaBlockToneMapping, MediaBlockTransitionFade,
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
                0x03080000 => {
                    let mut fx_colors = MediaBlockFxColors::default();
                    read_body_chunks(&mut fx_colors, r)?;

                    Ok(Self::FxColors(fx_colors))
                }
                0x03084000 => {
                    let mut camera_game = MediaBlockCameraGame::default();
                    read_body_chunks(&mut camera_game, r)?;

                    Ok(Self::CameraGame(camera_game))
                }
                0x030a1000 => {
                    let mut media_block_camera_path = MediaBlockCameraPath::default();
                    read_body_chunks(&mut media_block_camera_path, r)?;

                    Ok(Self::CameraPath(media_block_camera_path))
                }
                0x030a2000 => {
                    let mut media_block_camera_custom = MediaBlockCameraCustom::default();
                    read_body_chunks(&mut media_block_camera_custom, r)?;

                    Ok(Self::CameraCustom(media_block_camera_custom))
                }
                0x030a4000 => {
                    let mut camera_effect_shake = MediaBlockCameraEffectShake::default();
                    read_body_chunks(&mut camera_effect_shake, r)?;

                    Ok(Self::CameraEffectShake(camera_effect_shake))
                }
                0x030a5000 => {
                    let mut image = MediaBlockImage::default();
                    read_body_chunks(&mut image, r)?;

                    Ok(Self::Image(image))
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
                0x030a9000 => {
                    let mut trails = MediaBlockTrails::default();
                    read_body_chunks(&mut trails, r)?;

                    Ok(Self::Trails(trails))
                }
                0x030ab000 => {
                    let mut media_block_transition_fade = MediaBlockTransitionFade::default();
                    read_body_chunks(&mut media_block_transition_fade, r)?;

                    Ok(Self::TransitionFade(media_block_transition_fade))
                }
                0x03126000 => {
                    let mut dof = MediaBlockDof::default();
                    read_body_chunks(&mut dof, r)?;

                    Ok(Self::Dof(dof))
                }
                0x03127000 => {
                    let mut tone_mapping = MediaBlockToneMapping::default();
                    read_body_chunks(&mut tone_mapping, r)?;

                    Ok(Self::ToneMapping(tone_mapping))
                }
                0x0312a000 => {
                    let mut media_block_manialink = MediaBlockManialink::default();
                    read_body_chunks(&mut media_block_manialink, r)?;

                    Ok(Self::Manialink(media_block_manialink))
                }
                0x03165000 => {
                    let mut dirty_lens = MediaBlockDirtyLens::default();
                    read_body_chunks(&mut dirty_lens, r)?;

                    Ok(Self::DirtyLens(dirty_lens))
                }
                0x03186000 => {
                    let mut color_grading = MediaBlockColorGrading::default();
                    read_body_chunks(&mut color_grading, r)?;

                    Ok(Self::ColorGrading(color_grading))
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
                0x0329f000 => {
                    let mut entity = MediaBlockEntity::default();
                    read_body_chunks(&mut entity, r)?;

                    Ok(Self::Entity(entity))
                }
                _ => Err(Error::new(ErrorKind::Unsupported(format!(
                    "{class_id:08x?}"
                )))),
            }
        }
    }
}
