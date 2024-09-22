use std::io::Read;

use crate::{
    engines::game::{
        MediaBlockCameraCustom, MediaBlockColorGrading, MediaBlockFxColors, MediaBlockImage,
        MediaBlockInterface, MediaBlockManialink, MediaBlockSound, MediaBlockText,
        MediaBlockToneMapping, MediaBlockTriangles2D, MediaBlockTriangles3D,
    },
    read::{file::read_body_chunks, Error, IdStateMut, NodeStateMut, Reader},
};

use super::{MediaBlockFog, MediaBlockTransitionFade};

/// A media block.
pub enum MediaBlock {
    /// 2D triangles media block.
    Triangles2D(MediaBlockTriangles2D),
    /// 3D triangles media block.
    Triangles3D(MediaBlockTriangles3D),
    /// FX colors media block.
    FxColors(MediaBlockFxColors),
    /// Custom camera media block.
    CameraCustom(MediaBlockCameraCustom),
    /// Image media block.
    Image(MediaBlockImage),
    /// Sound media block.
    Sound(MediaBlockSound),
    /// Text media block.
    Text(MediaBlockText),
    /// Transition fade media block.
    TransitionFade(MediaBlockTransitionFade),
    /// Tone mapping media block.
    ToneMapping(MediaBlockToneMapping),
    /// Manialink media block.
    Manialink(MediaBlockManialink),
    /// Color grading media block.
    ColorGrading(MediaBlockColorGrading),
    /// Interface media block.
    Interface(MediaBlockInterface),
    /// Fog media block.
    Fog(MediaBlockFog),
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
            0x030a2000 => {
                let mut node = MediaBlockCameraCustom;
                read_body_chunks(&mut node, r)?;
                Self::CameraCustom(node)
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
            _ => return Err(Error),
        };

        Ok(node)
    }
}
