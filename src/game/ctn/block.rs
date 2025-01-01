//! Block.

use std::sync::Arc;

use crate::{game::WaypointSpecialProperty, Byte3, PitchYawRoll, Vec3};

use super::{BlockSkin, Direction, ElemColor, LightmapQuality};

/// Block.
#[derive(Default, Debug)]
pub struct Block {
    model_id: Arc<str>,
    pub(crate) ty: BlockType,
    pub(crate) has_flags: bool,
    mobil_index: u8,
    mobil_sub_index: u8,
    is_ground: bool,
    is_pillar: bool,
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

    /// Variant mobil index.
    pub const fn mobil_index(&self) -> u8 {
        self.mobil_index
    }

    /// Variant mobil sub index.
    pub const fn mobil_sub_index(&self) -> u8 {
        self.mobil_sub_index
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
}

/// Type of block.
#[derive(Debug)]
pub enum BlockType {
    /// Normal block.
    Normal {
        /// Cardinal direction.
        direction: Direction,
        /// Coordinate.
        coord: Byte3,
        /// Is ghost.
        is_ghost: bool,
    },
    /// Free block.
    Free {
        /// Position.
        position: Vec3,
        /// Rotation.
        rotation: PitchYawRoll,
    },
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::Normal {
            direction: Default::default(),
            coord: Default::default(),
            is_ghost: Default::default(),
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
            let direction = r.enum_u8::<Direction>()?;
            let coord = r.byte3()?;
            let flags = r.u32()?;

            if flags != 0xffffffff {
                self.has_flags = true;

                // unknown: 0b11001111100001000010111100110000

                self.mobil_index = (flags & 15) as u8;
                self.mobil_sub_index = ((flags >> 6) & 3) as u8;
                self.is_ground = (flags >> 12) & 1 != 0;
                self.is_pillar = (flags >> 14) & 1 != 0;

                if (flags >> 15) & 1 != 0 {
                    let _author = r.id()?;
                    self.skin = r.internal_node_ref_or_null::<BlockSkin>()?;
                }

                let _dunno = (flags >> 16) & 1 != 0;
                let _dunno = (flags >> 17) & 1 != 0;

                if (flags >> 19) & 1 != 0 || (flags >> 20) & 1 != 0 {
                    self.waypoint_special_property =
                        Some(r.internal_node_ref::<WaypointSpecialProperty>()?);
                }

                self.variant_index = ((flags >> 21) & 3) as u8;
                let is_ghost = (flags >> 28) & 1 != 0;
                let is_free = (flags >> 29) & 1 != 0;

                if !is_free {
                    self.ty = BlockType::Normal {
                        direction,
                        coord,
                        is_ghost,
                    };
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
