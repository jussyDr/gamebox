use std::io::Read;

use crate::{
    engines::game::{MediaBlockCameraCustom, MediaBlockTriangles3D},
    read::{file::read_body_chunks, IdStateMut, NodeStateMut, Reader},
    Error,
};

use super::{MediaBlockFog, MediaBlockTransitionFade};

/// A media block.
pub enum MediaBlock {
    /// 3D triangles media block.
    Triangles3D(MediaBlockTriangles3D),
    /// Custom camera media block.
    CameraCustom(MediaBlockCameraCustom),
    /// Transition fade media block.
    TransitionFade(MediaBlockTransitionFade),
    /// Fog media block.
    Fog(MediaBlockFog),
}

impl MediaBlock {
    pub(crate) fn read(
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<Self, Error> {
        let _index = r.u32()?;
        let class_id = r.u32()?;

        println!("{:02X?}", class_id);

        let node = match class_id {
            0x0304c000 => {
                let mut node = MediaBlockTriangles3D::default();
                read_body_chunks(&mut node, r)?;
                Self::Triangles3D(node)
            }
            0x030a2000 => {
                let mut node = MediaBlockCameraCustom;
                read_body_chunks(&mut node, r)?;
                Self::CameraCustom(node)
            }
            0x030AB000 => {
                let mut node = MediaBlockTransitionFade;
                read_body_chunks(&mut node, r)?;
                Self::TransitionFade(node)
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
