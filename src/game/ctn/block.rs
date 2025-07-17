use crate::{
    game::{WaypointSpecialProperty, ctn::BlockSkin},
    read::{BodyReader, Error},
};

pub struct Block<'a> {
    body: Body<'a>,
}

pub struct Body<'a> {
    info_id: &'a str,
    waypoint_property: Option<&'a WaypointSpecialProperty<'a>>,
}

impl Block<'_> {
    pub fn info_id(&self) -> &str {
        self.body.info_id
    }

    pub fn waypoint_property(&self) -> Option<&WaypointSpecialProperty> {
        self.body.waypoint_property
    }
}

impl<'a> Block<'a> {
    pub fn read_from_body(r: BodyReader<'a>) -> Result<Block<'a>, Error> {
        let body = Self::read_body(r)?;

        Ok(Self { body })
    }

    pub fn read_body(mut r: BodyReader<'a>) -> Result<Body<'a>, Error> {
        let info_id = r.id()?;
        let _direction = r.u8()?;
        let _coord = r.vec3_u8()?;
        let flags = r.u32()?;

        if flags & 0x00008000 != 0 {
            let _author = r.id()?;
            let _skin: &BlockSkin = r.node_ref()?;
        }

        if flags & 0x00080000 != 0 {
            todo!();
        }

        let waypoint_property = if flags & 0x00100000 != 0 {
            Some(r.node_ref()?)
        } else {
            None
        };

        if flags & 0x00040000 != 0 {
            todo!();
        }

        if flags & 0x00020000 != 0 {
            let _decal_id = r.id()?;
            let _decal_intensity = r.u32()?;
            let _decal_variant = r.u32()?;
        }

        if (flags & 0x20000000) != 0 {
            // let _kind = BlockKind::Free {
            //     position: Vec3::default(),
            //     yaw_pitch_roll: Vec3::default(),
            // };
        } else {
            // let _kind = BlockKind::Normal { direction, coord };
        }

        Ok(Body {
            info_id,
            waypoint_property,
        })
    }
}
