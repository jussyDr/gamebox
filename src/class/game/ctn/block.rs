//! Block.

use std::sync::Arc;

use crate::Vec3;

/// A block.
#[derive(Default)]
pub struct Block {
    id: Arc<str>,
    pub(crate) kind: BlockKind,
}

impl Block {
    /// Identifier.
    pub fn id(&self) -> &Arc<str> {
        &self.id
    }

    /// Kind.
    pub fn kind(&self) -> &BlockKind {
        &self.kind
    }
}

/// Block kind.
pub enum BlockKind {
    /// Normal block.
    Normal,
    /// Free block.
    Free {
        /// Position.
        position: Vec3,
        /// Rotation.
        yaw_pitch_roll: Vec3,
    },
}

impl Default for BlockKind {
    fn default() -> Self {
        Self::Normal
    }
}

mod read {
    use std::sync::Arc;

    use crate::{
        Vec3,
        class::game::{
            ctn::{
                block::{Block, BlockKind},
                block_skin::BlockSkin,
            },
            waypoint_special_property::WaypointSpecialProperty,
        },
        read::{BodyReader, Error, ReadBody},
    };

    impl ReadBody for Block {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            self.id = r.id()?;
            let _direction = r.u8()?;
            let _coord = r.repeat(3, |r| r.u8())?;
            let flags = r.u32()?;

            if flags & 0x00008000 != 0 {
                let _author: Arc<str> = r.id()?;
                let _skin: Arc<BlockSkin> = r.node_ref()?;
            }

            if flags & 0x00080000 != 0 {
                todo!();
            }

            if flags & 0x00100000 != 0 {
                let _waypoint_special_property: Arc<WaypointSpecialProperty> = r.node_ref()?;
            }

            if flags & 0x00040000 != 0 {
                todo!();
            }

            if flags & 0x00020000 != 0 {
                let _decal_id: Arc<str> = r.id()?;
                let _decal_intensity = r.u32()?;
                let _decal_variant = r.u32()?;
            }

            if (flags & 0x20000000) != 0 {
                self.kind = BlockKind::Free {
                    position: Vec3::default(),
                    yaw_pitch_roll: Vec3::default(),
                };
            } else {
                self.kind = BlockKind::Normal;
            }

            Ok(())
        }
    }
}
