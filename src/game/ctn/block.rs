//! Block.

use std::sync::Arc;

use crate::{game::WaypointSpecialProperty, Byte3, Vec3, YawPitchRoll};

use super::{BlockSkin, Direction, ElemColor, LightmapQuality};

/// Block.
#[derive(PartialEq, Default, Debug)]
pub struct Block {
    pub(crate) model_id: Arc<str>,
    pub(crate) has_flags: bool,
    pub(crate) mobil_index: u8,
    pub(crate) mobil_sub_index: u8,
    pub(crate) is_ground: bool,
    pub(crate) is_pillar: bool,
    pub(crate) skin: Option<Arc<BlockSkin>>,
    pub(crate) waypoint_property: Option<Arc<WaypointSpecialProperty>>,
    pub(crate) variant_index: u8,
    pub(crate) ty: BlockType,
    pub(crate) elem_color: ElemColor,
    pub(crate) lightmap_quality: LightmapQuality,
}

impl Block {
    /// Model identifier.
    pub const fn model_id(&self) -> &Arc<str> {
        &self.model_id
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

    /// Is pillar.
    pub const fn is_pillar(&self) -> bool {
        self.is_pillar
    }

    /// Skin.
    pub const fn skin(&self) -> Option<&Arc<BlockSkin>> {
        self.skin.as_ref()
    }

    /// Waypoint property.
    pub const fn waypoint_property(&self) -> Option<&Arc<WaypointSpecialProperty>> {
        self.waypoint_property.as_ref()
    }

    /// Block info variant index.
    pub const fn variant_index(&self) -> u8 {
        self.variant_index
    }

    /// Type.
    pub const fn ty(&self) -> &BlockType {
        &self.ty
    }

    /// Element color.
    pub const fn elem_color(&self) -> ElemColor {
        self.elem_color
    }

    /// Lightmap quality.
    pub const fn lightmap_quality(&self) -> LightmapQuality {
        self.lightmap_quality
    }

    /// Set element color.
    pub const fn set_elem_color(&mut self, elem_color: ElemColor) {
        self.elem_color = elem_color;
    }
}

/// Block type.
#[derive(PartialEq, Debug)]
pub enum BlockType {
    /// Normal.
    Normal {
        /// Direction.
        direction: Direction,
        /// Coordinate.
        coord: Byte3,
        /// Is ghost.
        is_ghost: bool,
    },
    /// Free.
    Free {
        /// Position.
        position: Vec3,
        /// Rotation.
        rotation: YawPitchRoll,
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
        read::{
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, ReadBody,
        },
        Vec3, YawPitchRoll,
    };

    use super::{Block, BlockType};

    impl ReadBody for Block {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            self.model_id = r.id()?;
            let direction = r.enum_u8()?;
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
                    self.skin = r.internal_node_ref_or_null()?;
                }

                let _dunno = (flags >> 16) & 1 != 0;
                let _dunno = (flags >> 17) & 1 != 0;

                if (flags >> 19) & 1 != 0 || (flags >> 20) & 1 != 0 {
                    self.waypoint_property = Some(r.internal_node_ref()?);
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
                        rotation: YawPitchRoll::default(),
                    };
                }
            }

            Ok(())
        }
    }
}

mod write {
    use std::{io::Write, sync::Arc};

    use crate::{
        game::ctn::Direction,
        write::{
            writable::WriteBody,
            writer::{IdStateMut, NodeStateMut},
            Error, Writer,
        },
        Byte3,
    };

    use super::{Block, BlockType};

    impl WriteBody for Block {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            w.id(&self.model_id)?;

            let (direction, coord, is_ghost, is_free) = match self.ty {
                BlockType::Normal {
                    direction,
                    coord,
                    is_ghost,
                } => (direction, coord, is_ghost, false),
                BlockType::Free { .. } => (Direction::North, Byte3::new(0, 0, 0), false, true),
            };

            w.u8(direction.into())?;
            w.byte3(coord)?;

            let mut flags = 0;

            flags |= self.mobil_index as u32;
            flags |= (self.mobil_sub_index as u32) << 6;

            if self.is_ground {
                flags |= 1 << 12;
            }

            if self.is_pillar {
                flags |= 1 << 14;
            }

            if self.skin.is_some() {
                flags |= 1 << 15;
            }

            if self.waypoint_property.is_some() {
                flags |= 1 << 20;
            }

            flags |= (self.variant_index as u32) << 21;

            if is_ghost {
                flags |= 1 << 28;
            }

            if is_free {
                flags |= 1 << 29;
            }

            w.u32(flags)?;

            if let Some(ref skin) = self.skin {
                w.id(&Arc::from("Nadeo"))?;
                w.internal_node_ref(skin)?;
            }

            if let Some(ref waypoint_property) = self.waypoint_property {
                w.internal_node_ref(waypoint_property)?;
            }

            Ok(())
        }
    }
}
