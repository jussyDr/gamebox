use crate::{
    game::{WaypointSpecialProperty, ctn::BlockSkin},
    read::{BodyReader, Error},
};

pub struct Block<'a> {
    info_id: &'a str,
    pub is_free: bool,
}

impl<'a> Block<'a> {
    pub fn read(r: &mut BodyReader<'a, '_>) -> Result<Self, Error> {
        let info_id = r.id()?;
        let _direction = r.u8()?;
        let _coord = r.vec3_u8()?;
        let flags = r.u32()?;

        if flags & 0x00008000 != 0 {
            let _author = r.id()?;
            let _skin = r.node_ref::<BlockSkin>()?;
        }

        if flags & 0x00080000 != 0 {
            todo!();
        }

        if flags & 0x00100000 != 0 {
            let _waypoint_special_property = r.node_ref::<WaypointSpecialProperty>()?;
        }

        if flags & 0x00040000 != 0 {
            todo!();
        }

        if flags & 0x00020000 != 0 {
            let _decal_id = r.id()?;
            let _decal_intensity = r.u32()?;
            let _decal_variant = r.u32()?;
        }

        let is_free = flags & 0x20000000 != 0;

        Ok(Self { info_id, is_free })
    }
}
