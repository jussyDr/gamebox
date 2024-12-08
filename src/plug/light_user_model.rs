//! Light user model.

use crate::{Class, Rgb};

/// Light user model.
#[derive(Default)]
pub struct LightUserModel {
    color: Rgb<f32>,
    intensity: f32,
    distance: f32,
    night_only: bool,
}

impl Class for LightUserModel {
    const CLASS_ID: u32 = 0x090f9000;
}

impl LightUserModel {
    /// Color.
    pub const fn color(&self) -> Rgb<f32> {
        self.color
    }

    /// Intensity.
    pub const fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Distance.
    pub const fn distance(&self) -> f32 {
        self.distance
    }

    /// Night only.
    pub const fn night_only(&self) -> bool {
        self.night_only
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::LightUserModel;

    impl ReadBody for LightUserModel {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for LightUserModel {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl LightUserModel {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            self.color = r.rgb()?;
            self.intensity = r.f32()?;
            self.distance = r.f32()?;
            let _point_emission_radius = r.f32()?;
            let _point_emission_length = r.f32()?;
            let _spot_inner_angle = r.f32()?;
            let _spot_outer_angle = r.f32()?;
            let _spot_emission_size_x = r.f32()?;
            let _spot_emission_size_y = r.f32()?;
            self.night_only = r.bool()?;

            Ok(())
        }
    }
}
