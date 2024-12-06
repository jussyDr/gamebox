//! Block.

use std::sync::Arc;

use crate::{game::WaypointSpecialProperty, Vec3};

use super::Direction;

/// A block.
#[derive(PartialEq, Default, Debug)]
pub struct Block {
    id: Arc<str>,
    direction: Direction,
    coord: Vec3<u8>,
    has_flags: bool,
    waypoint_special_property: Option<Arc<WaypointSpecialProperty>>,
}

impl Block {
    pub const fn id(&self) -> &Arc<str> {
        &self.id
    }

    pub const fn direction(&self) -> Direction {
        self.direction
    }

    pub const fn coord(&self) -> Vec3<u8> {
        self.coord
    }

    pub const fn waypoint_special_property(&self) -> Option<&Arc<WaypointSpecialProperty>> {
        self.waypoint_special_property.as_ref()
    }

    pub(crate) const fn has_flags(&self) -> bool {
        self.has_flags
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
    };

    use super::Block;

    impl ReadBody for Block {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            self.id = r.id()?;
            self.direction = r.enum_u8::<Direction>()?;
            self.coord.x = r.u8()?;
            self.coord.y = r.u8()?;
            self.coord.z = r.u8()?;
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
            }

            Ok(())
        }
    }
}
