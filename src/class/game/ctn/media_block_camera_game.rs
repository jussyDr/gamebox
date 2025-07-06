//! Media block camera game.

use crate::ClassId;

/// Media block camera game.
#[derive(Default)]
pub struct MediaBlockCameraGame;

impl ClassId for MediaBlockCameraGame {
    const CLASS_ID: u32 = 0x03084000;
}

mod read {
    use crate::{
        class::game::ctn::media_block_camera_game::MediaBlockCameraGame,
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
    };

    impl ReadBody for MediaBlockCameraGame {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaBlockCameraGame {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(7, Self::read_chunk_7)]
        }
    }

    impl MediaBlockCameraGame {
        fn read_chunk_7(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 4 {
                return Err(error_unknown_chunk_version(version));
            }

            let _start = r.f32()?;
            let _end = r.f32()?;
            let _game_cam = r.u32()?;
            let _clip_ent_id = r.u32()?;
            let _cam_position = r.vec3()?;
            let _cam_pitch_yaw_roll = r.vec3()?;
            let _cam_fov = r.f32()?;
            r.f32()?;
            r.f32()?;
            let _cam_near_clip_plane = r.f32()?;
            let _cam_far_clip_plane = r.f32()?;
            r.bool32()?;
            r.bool32()?;
            r.bool32()?;
            r.f32()?;
            r.u32()?;

            Ok(())
        }
    }
}
