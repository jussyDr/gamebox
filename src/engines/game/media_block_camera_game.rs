use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunks},
    Error, Reader,
};

/// A game camera media block.
pub struct MediaBlockCameraGame;

impl BodyChunks for MediaBlockCameraGame {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(7, |n, r| Self::read_chunk_7(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaBlockCameraGame {
    fn read_chunk_7<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 4 {
            return Err(Error);
        }

        let _start = r.f32()?;
        let _end = r.f32()?;
        let _game_cam = r.u32()?;
        let _clip_ent_id = r.u32()?;
        let _cam_position = r.vec3::<f32>()?;
        let _cam_pitch_yaw_roll = r.vec3::<f32>()?;
        let _cam_fov = r.f32()?;
        r.f32()?;
        r.f32()?;
        let _cam_near_clip_plane = r.f32()?;
        let _cam_far_clip_plane = r.f32()?;
        r.bool()?;
        r.bool()?;
        r.bool()?;
        r.f32()?;
        r.u32()?;

        Ok(())
    }
}
