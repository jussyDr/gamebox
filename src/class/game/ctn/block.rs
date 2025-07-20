use crate::{
    F32Vec3, U8Vec3,
    game::{
        WaypointSpecialProperty,
        ctn::{BlockSkin, Direction},
    },
    read::{BodyReader, Error},
};

pub struct Block<'a> {
    block_info_id: &'a str,
    pub(crate) transform: Transform,
    waypoint_property: Option<&'a WaypointSpecialProperty>,
    skin: Option<Skin<'a>>,
}

pub enum Transform {
    OnGrid {
        direction: Direction,
        coord: U8Vec3,
    },
    Free {
        position: F32Vec3,
        rotation: F32Vec3,
    },
}

pub struct Skin<'a> {
    author: &'a str,
    skin: &'a BlockSkin,
}

impl Block<'_> {
    pub fn block_info_id(&self) -> &str {
        self.block_info_id
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn waypoint_property(&self) -> Option<&WaypointSpecialProperty> {
        self.waypoint_property
    }

    pub fn skin(&self) -> Option<&Skin> {
        self.skin.as_ref()
    }
}

impl Skin<'_> {
    pub fn author(&self) -> &str {
        self.author
    }

    pub fn skin(&self) -> &BlockSkin {
        self.skin
    }
}

impl<'a> Block<'a> {
    pub fn read(r: &mut BodyReader<'a, '_>) -> Result<Self, Error> {
        let block_info_id = r.id()?;

        let direction = match r.u8()? {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            index => {
                return Err(Error::new(format!(
                    "unknown variant index of enum direction: {index}"
                )));
            }
        };

        let coord = r.vec3_u8()?;
        let flags = r.u32()?;

        let skin = if flags & 0x00008000 != 0 {
            let author = r.id()?;
            let skin = r.node_ref::<BlockSkin>()?;

            Some(Skin { author, skin })
        } else {
            None
        };

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

        let transform = if flags & 0x20000000 != 0 {
            Transform::Free {
                position: F32Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                rotation: F32Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            }
        } else {
            Transform::OnGrid { direction, coord }
        };

        Ok(Self {
            block_info_id,
            transform,
            waypoint_property,
            skin,
        })
    }
}
