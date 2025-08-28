use std::sync::Arc;

use crate::{
    Vec3, YawPitchRoll,
    game::{
        WaypointSpecialProperty,
        ctn::{ElemColor, FileRef, Ident, LightmapQuality, PhaseOffset},
    },
    read::{BodyReader, Error, ReadNode, Result, read_body_chunks},
};

pub struct AnchoredObject {
    chunk_2: Chunk2,
    chunk_4: Chunk4,
    chunk_5: Chunk5,
    pub(crate) elem_color: ElemColor,
    pub(crate) anim_offset: PhaseOffset,
    pub(crate) lightmap_quality: LightmapQuality,
}

struct Chunk2 {
    rotation: YawPitchRoll,
    coord: Vec3<u8>,
    position: Vec3<f32>,
    waypoint_property: Option<Arc<WaypointSpecialProperty>>,
    pivot_position: Vec3<f32>,
    scale: f32,
}

struct Chunk4;

struct Chunk5;

impl AnchoredObject {
    /// Rotation.
    pub fn rotation(&self) -> YawPitchRoll {
        self.chunk_2.rotation
    }

    /// Coordinate.
    pub fn coord(&self) -> Vec3<u8> {
        self.chunk_2.coord
    }

    /// Position.
    pub fn position(&self) -> Vec3<f32> {
        self.chunk_2.position
    }

    pub fn waypoint_property(&self) -> Option<&WaypointSpecialProperty> {
        self.chunk_2.waypoint_property.as_deref()
    }

    pub fn pivot_position(&self) -> Vec3<f32> {
        self.chunk_2.pivot_position
    }

    /// Scale.
    pub fn scale(&self) -> f32 {
        self.chunk_2.scale
    }

    pub fn elem_color(&self) -> ElemColor {
        self.elem_color
    }

    pub fn anim_offset(&self) -> PhaseOffset {
        self.anim_offset
    }

    /// Lightmap quality.
    pub fn lightmap_quality(&self) -> LightmapQuality {
        self.lightmap_quality
    }
}

impl ReadNode for AnchoredObject {
    const CLASS_ID: u32 = 0x03101000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            let chunk_2 = r.chunk(0x03101002, |r| {
                if r.u32()? != 8 {
                    return Err(Error::Internal("unknown chunk version".into()));
                }

                let _item_model = Ident::read(r)?;
                let rotation = r.yaw_pitch_roll()?;
                let coord = r.vec3_u8()?;
                let _anchor_tree_id = r.string_ref::<Option<Arc<str>>>()?;
                let position = r.vec3_f32()?;
                let waypoint_property = r.node_ref()?;
                let flags = r.u16()?;
                let pivot_position = r.vec3_f32()?;
                let scale = r.f32()?;

                if flags & 0x0004 != 0 {
                    let _file_ref = FileRef::read(r)?;
                }

                r.vec3_f32()?;
                r.vec3_f32()?;

                Ok(Chunk2 {
                    rotation,
                    coord,
                    position,
                    waypoint_property,
                    pivot_position,
                    scale,
                })
            })?;
            let chunk_4 = r.chunk_skippable(0x03101004, |r| {
                if r.u32()? != 0 {
                    return Err(Error::Internal("unknown chunk version".into()));
                }

                r.u32()?;

                Ok(Chunk4)
            })?;
            let chunk_5 = r.chunk_skippable(0x03101005, |r| {
                r.u32()?;
                r.u32()?;
                r.u8()?;

                Ok(Chunk5)
            })?;

            Ok(Self {
                chunk_2,
                chunk_4,
                chunk_5,
                elem_color: ElemColor::default(),
                anim_offset: PhaseOffset::default(),
                lightmap_quality: LightmapQuality::default(),
            })
        })
    }
}
