//! Block.

use std::sync::Arc;

use crate::{game::WaypointSpecialProperty, Vec3};

use super::Direction;

/// Block placed in a Challenge.
#[derive(Default)]
pub struct Block {
    block_model_id: Arc<str>,
    pub(crate) ty: BlockType,
    has_flags: bool,
    waypoint_special_property: Option<Arc<WaypointSpecialProperty>>,
}

impl Block {
    /// Identifier of the block's model.
    pub const fn block_model_id(&self) -> &Arc<str> {
        &self.block_model_id
    }

    /// Type.
    pub const fn ty(&self) -> &BlockType {
        &self.ty
    }

    /// Waypoint property of the block.
    pub const fn waypoint_special_property(&self) -> Option<&Arc<WaypointSpecialProperty>> {
        self.waypoint_special_property.as_ref()
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
        pos: Vec3<f32>,
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
        Vec3,
    };

    use super::{Block, BlockType};

    impl ReadBody for Block {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            self.block_model_id = r.id()?;
            let dir = r.enum_u8::<Direction>()?;
            let coord = r.vec3()?;
            let flags = r.u32()?;

            if flags != 0xffffffff {
                self.has_flags = true;

                if flags & 0x00008000 != 0 {
                    let _author = r.id()?;
                    let _skin = r.internal_node_ref_or_null::<BlockSkin>()?;
                }

                if flags & 0x00080000 != 0 || flags & 0x00100000 != 0 {
                    self.waypoint_special_property =
                        Some(r.internal_node_ref::<WaypointSpecialProperty>()?);
                }

                if flags & 0x20000000 == 0 {
                    self.ty = BlockType::Normal { dir, coord };
                } else {
                    self.ty = BlockType::Free {
                        pos: Vec3::default(),
                    };
                }
            }

            Ok(())
        }
    }
}
