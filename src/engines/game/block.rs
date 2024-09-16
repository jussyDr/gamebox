use std::io::Read;

use crate::{
    engines::{game::BlockSkin, game_data::WaypointSpecialProperty},
    read::{IdStateMut, IdStateRef, NodeStateMut, NodeStateRef, Reader},
    Direction, Error,
};

/// A block placed inside of a [Challenge](super::Challenge).
pub struct Block;

impl Block {
    pub(crate) fn read(
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<Self, Error> {
        let _name = r.id()?;
        let _direction = Direction::read_u8(r)?;
        let _coord = r.vec3::<u8>()?;
        let flags = r.u32()?;

        if flags & 0x00008000 != 0 {
            let _author = r.id()?;
            let _skin = r.node::<BlockSkin>()?;
        }

        if flags & 0x00100000 != 0 {
            let _waypoint_special_property = r.node::<WaypointSpecialProperty>()?;
        }

        Ok(Self)
    }

    pub(crate) fn read_inline(
        r: &mut Reader<impl Read, impl IdStateRef, impl NodeStateRef>,
    ) -> Result<Self, Error> {
        let _name = r.id_ref()?;
        let _direction = Direction::read_u8(r)?;
        let _coord = r.vec3::<u8>()?;
        let flags = r.u32()?;

        if flags & 0x00008000 != 0 {
            // let _author = r.id_ref()?;
            // let _skin = r.node::<BlockSkin>()?;

            panic!("{:02X?}", r.bytes(144)?);
        }

        if flags & 0x00100000 != 0 {
            // let _waypoint_special_property = r.node::<WaypointSpecialProperty>()?;

            panic!("{:02X?}", r.bytes(144)?);
        }

        Ok(Self)
    }
}
