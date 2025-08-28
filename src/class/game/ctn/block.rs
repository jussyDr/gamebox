use std::sync::Arc;

use crate::{
    Vec3, YawPitchRoll,
    game::{
        WaypointSpecialProperty,
        ctn::{BlockSkin, Direction, ElemColor, LightmapQuality},
    },
    plug::CharPhySpecialProperty,
    read::{BodyReader, Result},
};

/// A block placed inside a challenge.
pub struct Block {
    info_id: Arc<str>,
    skin: Option<Skin>,
    waypoint_property: Option<Arc<WaypointSpecialProperty>>,
    pub(crate) ty: Type,
    pub(crate) elem_color: ElemColor,
    pub(crate) lightmap_quality: LightmapQuality,
}

/// Block skin.
pub struct Skin {
    author: Arc<str>,
    skin: Arc<BlockSkin>,
}

/// Type of block.
pub enum Type {
    /// Normal block.
    Normal {
        /// Direction.
        direction: Direction,
        /// Coordinate.
        coord: Vec3<u8>,
    },
    /// Free block.
    Free {
        /// Absolute position.
        position: Vec3<f32>,
        /// Rotation.
        rotation: YawPitchRoll,
    },
}

impl Block {
    /// Block info identifier string.
    pub fn info_id(&self) -> &str {
        &self.info_id
    }

    /// Skin.
    pub fn skin(&self) -> Option<&Skin> {
        self.skin.as_ref()
    }

    /// Waypoint property.
    pub fn waypoint_property(&self) -> Option<&WaypointSpecialProperty> {
        self.waypoint_property.as_deref()
    }

    /// Type of block.
    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn elem_color(&self) -> ElemColor {
        self.elem_color
    }

    /// Lightmap quality.
    pub fn lightmap_quality(&self) -> LightmapQuality {
        self.lightmap_quality
    }
}

impl Block {
    pub(crate) fn read(r: &mut impl BodyReader) -> Result<Self> {
        let info_id = r.string_ref()?;
        let direction = r.enum8()?;
        let coord = r.vec3_u8()?;
        let flags = r.u32()?;

        let skin = if flags & 0x00008000 != 0 {
            let author = r.string_ref()?;
            let skin = r.node_ref()?;

            Some(Skin { author, skin })
        } else {
            None
        };

        if flags & 0x00080000 != 0 {
            let _phy_char_special_property = r.node_ref::<Arc<CharPhySpecialProperty>>()?;
        }

        let waypoint_property = if flags & 0x00100000 != 0 {
            Some(r.node_ref()?)
        } else {
            None
        };

        if flags & 0x00040000 != 0 {
            let _square_card_event_ids = r.list(|r| {
                r.u32()?;
                r.u32()?;
                r.list(|r| r.string_ref::<Arc<str>>())?;

                Ok(())
            })?;
        }

        if flags & 0x00020000 != 0 {
            let _decal_id = r.string_ref::<Arc<str>>()?;
            let _decal_intensity = r.u32()?;
            let _decal_variant = r.u32()?;
        }

        let ty = if flags & 0x20000000 == 0 {
            Type::Normal { direction, coord }
        } else {
            Type::Free {
                position: Vec3::default(),
                rotation: YawPitchRoll::default(),
            }
        };

        Ok(Self {
            info_id,
            skin,
            waypoint_property,
            ty,
            elem_color: ElemColor::default(),
            lightmap_quality: LightmapQuality::default(),
        })
    }
}
