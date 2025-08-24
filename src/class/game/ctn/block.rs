use std::sync::Arc;

use crate::{
    game::{
        WaypointSpecialProperty,
        ctn::{BlockSkin, Direction},
    },
    plug::CharPhySpecialProperty,
    read::{BodyReader, Result},
};

pub struct Block {
    pub(crate) is_free: bool,
}

impl Block {
    pub(crate) fn read(r: &mut impl BodyReader) -> Result<Self> {
        let _block_info_id = r.string_ref()?;
        let _direction = r.enum8::<Direction>()?;
        let _coord = r.vec3_u8()?;
        let flags = r.u32()?;

        if flags & 0x00008000 != 0 {
            let _author = r.string_ref()?;
            let _skin = r.node_ref::<Arc<BlockSkin>>()?;
        }

        if flags & 0x00080000 != 0 {
            let _phy_char_special_property = r.node_ref::<Arc<CharPhySpecialProperty>>()?;
        }

        if flags & 0x00100000 != 0 {
            let _waypoint_special_property = r.node_ref::<Arc<WaypointSpecialProperty>>()?;
        }

        if flags & 0x00040000 != 0 {
            let _square_card_event_ids = r.list(|r| {
                r.u32()?;
                r.u32()?;
                r.list(|r| r.string_ref())?;

                Ok(())
            })?;
        }

        if flags & 0x00020000 != 0 {
            let _decal_id = r.string_ref()?;
            let _decal_intensity = r.u32()?;
            let _decal_variant = r.u32()?;
        }

        let is_free = flags & 0x20000000 != 0;

        Ok(Self { is_free })
    }
}
