//! Media block camera game.

use crate::Class;

/// Media block camera game.
#[derive(Default)]
pub struct MediaBlockCameraGame;

impl Class for MediaBlockCameraGame {
    const CLASS_ID: u32 = 0x03084000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::MediaBlockCameraGame;

    impl ReadBody for MediaBlockCameraGame {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockCameraGame {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(7, Self::read_chunk_7)].into_iter()
        }
    }

    impl MediaBlockCameraGame {
        fn read_chunk_7<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 4 {
                return Err(Error::chunk_version(version));
            }

            let _start = r.f32()?;
            let _end = r.f32()?;
            let _game_cam = r.u32()?;
            let _clip_ent_id = r.u32()?;
            let _cam_position = r.vec3::<f32>()?;
            let _cam_rotation = r.pitch_yaw_roll()?;
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
}
