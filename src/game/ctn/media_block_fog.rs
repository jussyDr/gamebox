//! Media block fog.

use bytemuck::cast;
use ordered_float::OrderedFloat;

use crate::{Class, OrderedRgbFloat, RgbFloat};

/// A media block fog.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
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
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Key {
    time: OrderedFloat<f32>,
    intensity: OrderedFloat<f32>,
    sky_intensity: OrderedFloat<f32>,
    distance: OrderedFloat<f32>,
    coefficient: OrderedFloat<f32>,
    color: OrderedRgbFloat,
    clouds_opacity: OrderedFloat<f32>,
    clouds_speed: OrderedFloat<f32>,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time.0
    }

    /// Intensity.
    pub const fn intensity(&self) -> f32 {
        self.intensity.0
    }

    /// Sky intensity.
    pub const fn sky_intensity(&self) -> f32 {
        self.sky_intensity.0
    }

    /// Distance.
    pub const fn distance(&self) -> f32 {
        self.distance.0
    }

    /// Coefficient.
    pub const fn coefficient(&self) -> f32 {
        self.coefficient.0
    }

    /// Color
    pub fn color(&self) -> RgbFloat {
        cast(self.color)
    }

    /// Clouds opacity.
    pub const fn clouds_opacity(&self) -> f32 {
        self.clouds_opacity.0
    }

    /// Clouds speed.
    pub const fn clouds_speed(&self) -> f32 {
        self.clouds_speed.0
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
                let color = r.rgb_float_ordered()?;
                let clouds_opacity = r.f32()?;
                let clouds_speed = r.f32()?;

                Ok(Key {
                    time: OrderedFloat(time),
                    intensity: OrderedFloat(intensity),
                    sky_intensity: OrderedFloat(sky_intensity),
                    distance: OrderedFloat(distance),
                    coefficient: OrderedFloat(coefficient),
                    color,
                    clouds_opacity: OrderedFloat(clouds_opacity),
                    clouds_speed: OrderedFloat(clouds_speed),
                })
            })?;

            Ok(())
        }
    }
}
