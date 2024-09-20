use std::io::Read;

use crate::{
    read::{
        readable::{BodyChunk, BodyChunks},
        Reader,
    },
    Error,
};

enum Interpolation {
    None,
    Hermite,
    Linear,
    FixedTangent,
}

impl Interpolation {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let interpolation = match r.u32()? {
            0 => Self::None,
            1 => Self::Hermite,
            2 => Self::Linear,
            3 => Self::FixedTangent,
            _ => return Err(Error),
        };

        Ok(interpolation)
    }
}

/// A custom camera media block.
pub struct MediaBlockCameraCustom;

impl BodyChunks for MediaBlockCameraCustom {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(6, |n, r| Self::read_chunk_6(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaBlockCameraCustom {
    fn read_chunk_6<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 3 {
            return Err(Error);
        }

        let _keys = r.list(|r| {
            let _time = r.f32()?;
            let _interpolation = Interpolation::read(r)?;
            let _anchor_rot = r.bool()?;
            let _anchor = r.u32()?;
            let _anchor_vis = r.bool()?;
            let _target = r.u32()?;
            let _position = r.vec3::<f32>()?;
            let _pitch_yaw_roll = r.vec3::<f32>()?;
            let _fov = r.f32()?;
            let _target_position = r.vec3::<f32>()?;
            let _near_z = r.f32()?;
            let _position = r.vec3::<f32>()?;
            let _pitch_yaw_roll = r.vec3::<f32>()?;
            let _fov = r.f32()?;
            let _target_position = r.vec3::<f32>()?;
            let _near_z = r.f32()?;
            let _position = r.vec3::<f32>()?;
            let _pitch_yaw_roll = r.vec3::<f32>()?;
            let _fov = r.f32()?;
            let _target_position = r.vec3::<f32>()?;
            let _near_z = r.f32()?;

            Ok(())
        })?;

        Ok(())
    }
}
