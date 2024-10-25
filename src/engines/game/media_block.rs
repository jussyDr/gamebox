use std::io::Read;

use crate::{
    engines::game::{
        MediaBlockCameraCustom, MediaBlockCameraEffectShake, MediaBlockCameraGame,
        MediaBlockColorGrading, MediaBlockDirtyLens, MediaBlockFxColors, MediaBlockImage,
        MediaBlockInterface, MediaBlockManialink, MediaBlockSound, MediaBlockText,
        MediaBlockToneMapping, MediaBlockTriangles2D, MediaBlockTriangles3D,
    },
    read::{file::read_body_chunks, Error, IdStateMut, NodeStateMut, Reader},
};

use super::{MediaBlockDOF, MediaBlockEntity, MediaBlockFog, MediaBlockTransitionFade};

/// A media block.
pub enum MediaBlock {
    /// 2D triangles media block.
    Triangles2D(MediaBlockTriangles2D),
    /// 3D triangles media block.
    Triangles3D(MediaBlockTriangles3D),
    /// FX colors media block.
    FxColors(MediaBlockFxColors),
    /// Camera game media block.
    CameraGame(MediaBlockCameraGame),
    /// Custom camera media block.
    CameraCustom(MediaBlockCameraCustom),
    /// Shake camera effect media block.
    CameraEffectShake(MediaBlockCameraEffectShake),
    /// Image media block.
    Image(MediaBlockImage),
    /// Sound media block.
    Sound(MediaBlockSound),
    /// Text media block.
    Text(MediaBlockText),
    /// Transition fade media block.
    TransitionFade(MediaBlockTransitionFade),
    /// DOF media block.
    DOF(MediaBlockDOF),
    /// Tone mapping media block.
    ToneMapping(MediaBlockToneMapping),
    /// Manialink media block.
    Manialink(MediaBlockManialink),
    /// Dirty lens media block.
    DirtyLens(MediaBlockDirtyLens),
    /// Color grading media block.
    ColorGrading(MediaBlockColorGrading),
    /// Interface media block.
    Interface(MediaBlockInterface),
    /// Fog media block.
    Fog(MediaBlockFog),
    /// Entity media block.
    Entity(MediaBlockEntity),
}

impl MediaBlock {
    pub(crate) fn read(
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<Self, Error> {
        let _index = r.u32()?;
        let class_id = r.u32()?;

        let node = match class_id {
            0x0304b000 => {
                let mut node = MediaBlockTriangles2D::default();
                read_body_chunks(&mut node, r)?;
                Self::Triangles2D(node)
            }
            0x0304c000 => {
                let mut node = MediaBlockTriangles3D::default();
                read_body_chunks(&mut node, r)?;
                Self::Triangles3D(node)
            }
            0x03080000 => {
                let mut node = MediaBlockFxColors;
                read_body_chunks(&mut node, r)?;
                Self::FxColors(node)
            }
            0x03084000 => {
                let mut node = MediaBlockCameraGame;
                read_body_chunks(&mut node, r)?;
                Self::CameraGame(node)
            }
            0x030a2000 => {
                let mut node = MediaBlockCameraCustom;
                read_body_chunks(&mut node, r)?;
                Self::CameraCustom(node)
            }
            0x030a4000 => {
                let mut node = MediaBlockCameraEffectShake;
                read_body_chunks(&mut node, r)?;
                Self::CameraEffectShake(node)
            }
            0x030a5000 => {
                let mut node = MediaBlockImage;
                read_body_chunks(&mut node, r)?;
                Self::Image(node)
            }
            0x030a7000 => {
                let mut node = MediaBlockSound;
                read_body_chunks(&mut node, r)?;
                Self::Sound(node)
            }
            0x030a8000 => {
                let mut node = MediaBlockText;
                read_body_chunks(&mut node, r)?;
                Self::Text(node)
            }
            0x030ab000 => {
                let mut node = MediaBlockTransitionFade;
                read_body_chunks(&mut node, r)?;
                Self::TransitionFade(node)
            }
            0x03126000 => {
                let mut node = MediaBlockDOF;
                read_body_chunks(&mut node, r)?;
                Self::DOF(node)
            }
            0x03127000 => {
                let mut node = MediaBlockToneMapping;
                read_body_chunks(&mut node, r)?;
                Self::ToneMapping(node)
            }
            0x0312a000 => {
                let mut node = MediaBlockManialink;
                read_body_chunks(&mut node, r)?;
                Self::Manialink(node)
            }
            0x03165000 => {
                let mut node = MediaBlockDirtyLens;
                read_body_chunks(&mut node, r)?;
                Self::DirtyLens(node)
            }
            0x03186000 => {
                let mut node = MediaBlockColorGrading;
                read_body_chunks(&mut node, r)?;
                Self::ColorGrading(node)
            }
            0x03195000 => {
                let mut node = MediaBlockInterface;
                read_body_chunks(&mut node, r)?;
                Self::Interface(node)
            }
            0x03199000 => {
                let mut node = MediaBlockFog;
                read_body_chunks(&mut node, r)?;
                Self::Fog(node)
            }
            0x0329f000 => {
                let mut node = MediaBlockEntity;
                read_body_chunks(&mut node, r)?;
                Self::Entity(node)
            }
            _ => return Err(Error),
        };

        Ok(node)
    }
}
