use std::io::Read;

use crate::{
    engines::{game::BlockSkin, game_data::WaypointSpecialProperty},
    read::{Error, IdStateMut, NodeStateMut, Reader},
    Direction, Vec3,
};

use super::challenge::{LightmapQuality, MapElemColor};

/// A block placed inside of a [Challenge](super::Challenge).
pub struct Block {
    pub(crate) is_free: bool,
    pub(crate) absolute_position_in_map: Vec3<f32>,
    pub(crate) pitch_yaw_roll: Vec3<f32>,
    pub(crate) color: MapElemColor,
    pub(crate) lightmap_quality: LightmapQuality,
}

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

        if flags & 0x00020000 != 0 {
            let _decal_id = r.id()?;
            let _decal_intensity = r.u32()?;
            let _decal_variant = r.u32()?;
        }

        let is_free = flags & 0x20000000 != 0;

        Ok(Self {
            is_free,
            absolute_position_in_map: Vec3::default(),
            pitch_yaw_roll: Vec3::default(),
            color: MapElemColor::default(),
            lightmap_quality: LightmapQuality::default(),
        })
    }
}
