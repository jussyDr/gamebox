//! Block.

use std::sync::Arc;

use crate::{game::WaypointSpecialProperty, PitchYawRoll, Vec3};

use super::{BlockSkin, Direction, ElemColor, LightmapQuality};

/// Block placed in a Challenge.
#[derive(Default)]
pub struct Block {
    model_id: Arc<str>,
    pub(crate) ty: BlockType,
    has_flags: bool,
    is_ground: bool,
    skin: Option<Arc<BlockSkin>>,
    waypoint_special_property: Option<Arc<WaypointSpecialProperty>>,
    variant_index: u8,
    pub(crate) elem_color: ElemColor,
    pub(crate) lightmap_quality: LightmapQuality,
}

impl Block {
    /// Model identifier.
    pub const fn model_id(&self) -> &Arc<str> {
        &self.model_id
    }

    /// Type.
    pub const fn ty(&self) -> &BlockType {
        &self.ty
    }

    /// Is ground.
    pub const fn is_ground(&self) -> bool {
        self.is_ground
    }

    /// Skin.
    pub const fn skin(&self) -> Option<&Arc<BlockSkin>> {
        self.skin.as_ref()
    }

    /// Waypoint property of the block.
    pub const fn waypoint_special_property(&self) -> Option<&Arc<WaypointSpecialProperty>> {
        self.waypoint_special_property.as_ref()
    }

    /// Block info variant index.
    pub const fn variant_index(&self) -> u8 {
        self.variant_index
    }

    /// Element color.
    pub const fn elem_color(&self) -> ElemColor {
        self.elem_color
    }

    /// Lightmap quality.
    pub const fn lightmap_quality(&self) -> LightmapQuality {
        self.lightmap_quality
    }

    pub(crate) const fn has_flags(&self) -> bool {
        self.has_flags
    }
}

/// Type of block.
pub enum BlockType {
    /// Normal block.
    Normal {
        /// Cardinal direction.
        dir: Direction,
        /// Coordinate.
        coord: Vec3<u8>,
    },
    /// Free block.
    Free {
        /// Position.
        position: Vec3<f32>,
        /// Rotation.
        rotation: PitchYawRoll,
    },
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::Normal {
            dir: Direction::default(),
            coord: Vec3::default(),
        }
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::{
            ctn::{block_skin::BlockSkin, Direction},
            waypoint_special_property::WaypointSpecialProperty,
        },
        read::{
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, ReadBody,
        },
        PitchYawRoll, Vec3,
    };

    use super::{Block, BlockType};

    impl ReadBody for Block {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            self.model_id = r.id()?;
            let dir = r.enum_u8::<Direction>()?;
            let coord = r.vec3()?;
            let flags = r.u32()?;

            if flags != 0xffffffff {
                self.has_flags = true;

                self.is_ground = flags & 0x00001000 != 0;

                if flags & 0x00008000 != 0 {
                    let _author = r.id()?;
                    self.skin = r.internal_node_ref_or_null::<BlockSkin>()?;
                }

                if flags & 0x00080000 != 0 || flags & 0x00100000 != 0 {
                    self.waypoint_special_property =
                        Some(r.internal_node_ref::<WaypointSpecialProperty>()?);
                }

                self.variant_index = ((flags >> 21) & 0x0000003f) as u8;

                if flags & 0x20000000 == 0 {
                    self.ty = BlockType::Normal { dir, coord };
                } else {
                    self.ty = BlockType::Free {
                        position: Vec3::default(),
                        rotation: PitchYawRoll::default(),
                    };
                }
            }

            Ok(())
        }
    }
}
