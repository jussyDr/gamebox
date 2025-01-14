//! Anchored object.

use std::sync::Arc;

use crate::{
    game::WaypointSpecialProperty, read::reader::FromVariant, Byte3, Class, FileRef, Vec3,
    YawPitchRoll,
};

use super::{ElemColor, LightmapQuality};

/// Anchored object.
#[derive(PartialEq, Default, Debug)]
pub struct AnchoredObject {
    model_id: Arc<str>,
    rotation: YawPitchRoll,
    unit_coord: Byte3,
    position: Vec3,
    waypoint_property: Option<WaypointSpecialProperty>,
    variant_index: u8,
    pivot_position: Vec3,
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

    /// Unit coordinate.
    pub const fn unit_coord(&self) -> Byte3 {
        self.unit_coord
    }

    /// Position.
    pub const fn position(&self) -> Vec3 {
        self.position
    }

    /// Waypoint property.
    pub const fn waypoint_property(&self) -> Option<&WaypointSpecialProperty> {
        self.waypoint_property.as_ref()
    }

    /// Variant index.
    pub const fn variant_index(&self) -> u8 {
        self.variant_index
    }

    /// Pivot position.
    pub const fn pivot_position(&self) -> Vec3 {
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
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum PhaseOffset {
    /// No offset.
    #[default]
    None,
    /// One eighth.
    One8th,
    /// Two eights.
    Two8th,
    /// Three eights.
    Three8th,
    /// Four eights.
    Four8th,
    /// Five eights.
    Five8th,
    /// Six eights.
    Six8th,
    /// Seven eights.
    Seven8th,
}

impl FromVariant<u8> for PhaseOffset {
    fn from_variant(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::One8th),
            2 => Some(Self::Two8th),
            3 => Some(Self::Three8th),
            4 => Some(Self::Four8th),
            5 => Some(Self::Five8th),
            6 => Some(Self::Six8th),
            7 => Some(Self::Seven8th),
            _ => None,
        }
    }
}

impl From<PhaseOffset> for u8 {
    fn from(value: PhaseOffset) -> u8 {
        match value {
            PhaseOffset::None => 0,
            PhaseOffset::One8th => 1,
            PhaseOffset::Two8th => 2,
            PhaseOffset::Three8th => 3,
            PhaseOffset::Four8th => 4,
            PhaseOffset::Five8th => 5,
            PhaseOffset::Six8th => 6,
            PhaseOffset::Seven8th => 7,
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
            #![allow(clippy::redundant_closure)]
            [
                BodyChunk::normal(2, Self::read_chunk_2),
                BodyChunk::skippable(4, |s, r| Self::read_chunk_4(s, r)),
                BodyChunk::skippable(5, |s, r| Self::read_chunk_5(s, r)),
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
            self.unit_coord = r.byte3()?;
            let _anchor_tree_id = r.id_or_null()?;
            self.position = r.vec3()?;
            self.waypoint_property = r.node_or_null::<WaypointSpecialProperty>()?;
            let flags = r.u16()?;
            self.variant_index = ((flags >> 8) & 255) as u8;
            let _show = ((flags >> 12) & 1) != 0;
            self.pivot_position = r.vec3()?;
            self.scale = r.f32()?;

            if (flags >> 2) & 1 != 0 {
                self.skin = r.file_ref_or_null()?;
            }

            r.vec3()?;
            r.vec3()?;

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

mod write {
    use std::io::Write;

    use crate::{
        write::{
            writable::{write_body_chunks, WriteBody},
            writer::{IdStateMut, NodeStateMut},
            BodyChunk, BodyChunks, Error, Writer,
        },
        Vec3,
    };

    use super::AnchoredObject;

    impl WriteBody for AnchoredObject {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for AnchoredObject {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            #![allow(clippy::redundant_closure)]
            [
                BodyChunk::normal(2, Self::write_chunk_2),
                BodyChunk::skippable(4, |s, w| Self::write_chunk_4(s, w)),
                BodyChunk::skippable(5, |s, w| Self::write_chunk_5(s, w)),
            ]
            .into_iter()
        }
    }

    impl AnchoredObject {
        fn write_chunk_2(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            w.u32(8)?;
            w.id(&self.model_id)?;
            w.u32(0x1a)?;
            w.id_or_null(None)?;
            w.yaw_pitch_roll(self.rotation)?;
            w.byte3(self.unit_coord)?;
            w.id_or_null(None)?;
            w.vec3(self.position)?;
            w.node_or_null(self.waypoint_property.as_ref())?;

            let mut flags = 0;

            if self.skin.is_some() {
                flags |= 1 << 2;
            }

            flags |= (self.variant_index as u16) << 8;

            w.u16(flags)?;
            w.vec3(self.pivot_position)?;
            w.f32(self.scale)?;

            if let Some(ref skin) = self.skin {
                w.file_ref(skin)?;
            }

            w.vec3(Vec3::new(0.0, 0.0, 0.0))?;
            w.vec3(Vec3::new(-1.0, -1.0, -1.0))?;

            Ok(())
        }

        fn write_chunk_4<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;
            w.u32(0xffffffff)?;

            Ok(())
        }

        fn write_chunk_5<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(1)?;
            w.u32(4)?;
            w.u8(0)?;

            Ok(())
        }
    }
}
