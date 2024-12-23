//! Anchored object.

use std::sync::Arc;

use crate::{game::WaypointSpecialProperty, Class, FileRef, Vec3, YawPitchRoll};

use super::{ElemColor, LightmapQuality};

/// An anchored object.
#[derive(Default)]
pub struct AnchoredObject {
    model_id: Arc<str>,
    rotation: YawPitchRoll,
    unit_coord: Vec3<u8>,
    position: Vec3<f32>,
    waypoint_property: Option<WaypointSpecialProperty>,
    pivot_position: Vec3<f32>,
    scale: f32,
    skin: Option<FileRef>,
    pub(crate) elem_color: ElemColor,
    pub(crate) anim_offset: PhaseOffset,
    pub(crate) skin_effect: Option<FileRef>,
    pub(crate) lightmap_quality: LightmapQuality,
}

impl Class for AnchoredObject {
    const CLASS_ID: u32 = 0x03101000;
}

impl AnchoredObject {
    /// Item model identifier.
    pub const fn model_id(&self) -> &Arc<str> {
        &self.model_id
    }

    /// Rotation.
    pub const fn rotation(&self) -> YawPitchRoll {
        self.rotation
    }

    /// Block unit coordinate.
    pub const fn unit_coord(&self) -> Vec3<u8> {
        self.unit_coord
    }

    /// Position.
    pub const fn position(&self) -> Vec3<f32> {
        self.position
    }

    /// Waypoint property.
    pub const fn waypoint_property(&self) -> Option<&WaypointSpecialProperty> {
        self.waypoint_property.as_ref()
    }

    /// Pivot position.
    pub const fn pivot_position(&self) -> Vec3<f32> {
        self.pivot_position
    }

    /// Scale.
    pub const fn scale(&self) -> f32 {
        self.scale
    }

    /// Skin.
    pub const fn skin(&self) -> Option<&FileRef> {
        self.skin.as_ref()
    }

    /// Element color.
    pub const fn elem_color(&self) -> ElemColor {
        self.elem_color
    }

    /// Animation offset.
    pub const fn anim_offset(&self) -> PhaseOffset {
        self.anim_offset
    }

    /// Skin effect.
    pub const fn skin_effect(&self) -> Option<&FileRef> {
        self.skin_effect.as_ref()
    }

    /// Lightmap quality.
    pub const fn lightmap_quality(&self) -> LightmapQuality {
        self.lightmap_quality
    }
}

/// Animation phase offset.
#[derive(Clone, Copy, Default)]
pub enum PhaseOffset {
    /// No offset.
    #[default]
    None,
    /// No offset.
    One8th,
    /// No offset.
    Two8th,
    /// No offset.
    Three8th,
    /// No offset.
    Four8th,
    /// No offset.
    Five8th,
    /// No offset.
    Six8th,
    /// No offset.
    Seven8th,
}

impl TryFrom<u8> for PhaseOffset {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::One8th),
            2 => Ok(Self::Two8th),
            3 => Ok(Self::Three8th),
            4 => Ok(Self::Four8th),
            5 => Ok(Self::Five8th),
            6 => Ok(Self::Six8th),
            7 => Ok(Self::Seven8th),
            _ => Err(()),
        }
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::WaypointSpecialProperty,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::AnchoredObject;

    impl ReadBody for AnchoredObject {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for AnchoredObject {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(2, Self::read_chunk_2),
                BodyChunk::skippable(4, Self::read_chunk_4),
                BodyChunk::skippable(5, Self::read_chunk_5),
            ]
            .into_iter()
        }
    }

    impl AnchoredObject {
        fn read_chunk_2(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 8 {
                return Err(Error::chunk_version(version));
            }

            self.model_id = r.id()?;
            let _model_collection = r.id_or_null()?;
            let _model_author = r.id_or_null()?;
            self.rotation = r.yaw_pitch_roll()?;
            self.unit_coord = r.vec3()?;
            let _anchor_tree_id = r.id_or_null()?;
            self.position = r.vec3()?;
            self.waypoint_property = r.node_or_null::<WaypointSpecialProperty>()?;
            let flags = r.u16()?;
            self.pivot_position = r.vec3()?;
            self.scale = r.f32()?;

            if flags & 4 != 0 {
                self.skin = r.pack_desc_or_null()?;
            }

            r.vec3::<f32>()?;
            r.vec3::<f32>()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u8()?;

            Ok(())
        }
    }
}
