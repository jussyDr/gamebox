//! Media block fog.

use crate::{Class, Rgb};

/// A media block fog.
#[derive(Default)]
pub struct MediaBlockFog {
    keys: Vec<Key>,
}

impl Class for MediaBlockFog {
    const CLASS_ID: u32 = 0x03199000;
}

impl MediaBlockFog {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Fog media block key.
pub struct Key {
    time: f32,
    intensity: f32,
    sky_intensity: f32,
    distance: f32,
    coefficient: f32,
    color: Rgb<f32>,
    clouds_opacity: f32,
    clouds_speed: f32,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time
    }

    /// Intensity.
    pub const fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Sky intensity.
    pub const fn sky_intensity(&self) -> f32 {
        self.sky_intensity
    }

    /// Distance.
    pub const fn distance(&self) -> f32 {
        self.distance
    }

    /// Coefficient.
    pub const fn coefficient(&self) -> f32 {
        self.coefficient
    }

    /// Color
    pub const fn color(&self) -> Rgb<f32> {
        self.color
    }

    /// Clouds opacity.
    pub const fn clouds_opacity(&self) -> f32 {
        self.clouds_opacity
    }

    /// Clouds speed.
    pub const fn clouds_speed(&self) -> f32 {
        self.clouds_speed
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Key, MediaBlockFog};

    impl ReadBody for MediaBlockFog {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockFog {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl MediaBlockFog {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            self.keys = r.list(|r| {
                let time = r.f32()?;
                let intensity = r.f32()?;
                let sky_intensity = r.f32()?;
                let distance = r.f32()?;
                let coefficient = r.f32()?;
                let color = r.rgb()?;
                let clouds_opacity = r.f32()?;
                let clouds_speed = r.f32()?;

                Ok(Key {
                    time,
                    intensity,
                    sky_intensity,
                    distance,
                    coefficient,
                    color,
                    clouds_opacity,
                    clouds_speed,
                })
            })?;

            Ok(())
        }
    }
}
