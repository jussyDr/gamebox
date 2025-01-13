//! Media block camera custom.

use ordered_float::OrderedFloat;

use crate::{read::reader::FromVariant, Class};

/// Custom camera media block.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockCameraCustom {
    keys: Vec<Key>,
}

impl Class for MediaBlockCameraCustom {
    const CLASS_ID: u32 = 0x030a2000;
}

impl MediaBlockCameraCustom {
    /// Keys
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Custom camera media block key.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Key {
    time: OrderedFloat<f32>,
    interpolation: Interpolation,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time.0
    }

    /// Interpolation.
    pub const fn interpolation(&self) -> Interpolation {
        self.interpolation
    }
}

/// Interpolation.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Interpolation {
    /// None.
    None,
    /// Hermite.
    Hermite,
    /// Linear.
    Linear,
    /// Fixed tangent.
    FixedTangent,
}

impl FromVariant<u32> for Interpolation {
    fn from_variant(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::Hermite),
            2 => Some(Self::Linear),
            3 => Some(Self::FixedTangent),
            _ => None,
        }
    }
}

impl Into<u32> for Interpolation {
    fn into(self) -> u32 {
        match self {
            Self::None => 0,
            Self::Hermite => 1,
            Self::Linear => 2,
            Self::FixedTangent => 3,
        }
    }
}

mod read {
    use std::io::{Read, Seek};

    use ordered_float::OrderedFloat;

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Key, MediaBlockCameraCustom};

    impl ReadBody for MediaBlockCameraCustom {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockCameraCustom {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(6, Self::read_chunk_6)].into_iter()
        }
    }

    impl MediaBlockCameraCustom {
        fn read_chunk_6<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 3 | 4) {
                return Err(Error::chunk_version(version));
            }

            self.keys = r.list(|r| {
                let time = r.f32()?;
                let interpolation = r.enum_u32()?;
                let _anchor_rot = r.bool()?;
                let _anchor = r.u32()?;
                let _anchor_vis = r.bool()?;
                let _target = r.u32()?;
                let _position = r.vec3()?;
                let _pitch_yaw_roll = r.vec3()?;
                let _fov = r.f32()?;
                let _target_position = r.vec3()?;
                let _near_z = r.f32()?;
                let _position = r.vec3()?;
                let _pitch_yaw_roll = r.vec3()?;

                if version >= 4 {
                    r.u32()?;
                }

                let _fov = r.f32()?;
                let _target_position = r.vec3()?;
                let _near_z = r.f32()?;
                let _position = r.vec3()?;
                let _pitch_yaw_roll = r.vec3()?;

                if version >= 4 {
                    r.u32()?;
                }

                let _fov = r.f32()?;
                let _target_position = r.vec3()?;
                let _near_z = r.f32()?;

                if version >= 4 {
                    r.u32()?;
                }

                Ok(Key {
                    time: OrderedFloat(time),
                    interpolation,
                })
            })?;

            Ok(())
        }
    }
}

mod write {
    use std::io::Write;

    use crate::write::{
        writable::{write_body_chunks, WriteBody},
        writer::{IdStateMut, NodeStateMut},
        BodyChunk, BodyChunks, Error, Writer,
    };

    use super::MediaBlockCameraCustom;

    impl WriteBody for MediaBlockCameraCustom {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for MediaBlockCameraCustom {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [BodyChunk::normal(6, Self::write_chunk_6)].into_iter()
        }
    }

    impl MediaBlockCameraCustom {
        fn write_chunk_6<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(4)?;

            w.list(&self.keys, |w, key| {
                w.f32(key.time.0)?;
                w.u32(key.interpolation.into())?;

                todo!();

                Ok(())
            })?;

            Ok(())
        }
    }
}
