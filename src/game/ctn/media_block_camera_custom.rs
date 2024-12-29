//! Media block camera custom.

use crate::Class;

/// Custom camera media block.
#[derive(Default)]
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
pub struct Key {
    time: f32,
    interpolation: Option<Interpolation>,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time
    }

    /// Interpolation.
    pub const fn interpolation(&self) -> Option<Interpolation> {
        self.interpolation
    }
}

/// Interpolation.
#[derive(Clone, Copy)]
pub enum Interpolation {
    /// Hermite.
    Hermite,
    /// Linear.
    Linear,
    /// Fixed tangent.
    FixedTangent,
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Interpolation, Key, MediaBlockCameraCustom};

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
                let interpolation = match r.u32()? {
                    0 => None,
                    1 => Some(Interpolation::Hermite),
                    2 => Some(Interpolation::Linear),
                    3 => Some(Interpolation::FixedTangent),
                    _ => panic!(),
                };
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
                    time,
                    interpolation,
                })
            })?;

            Ok(())
        }
    }
}
