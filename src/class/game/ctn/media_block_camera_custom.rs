//! Media block camera custom.

use crate::ClassId;

/// Media block camera custom.
#[derive(Default)]
pub struct MediaBlockCameraCustom;

impl ClassId for MediaBlockCameraCustom {
    const CLASS_ID: u32 = 0x030a2000;
}

mod read {
    use crate::{
        class::game::ctn::media_block_camera_custom::MediaBlockCameraCustom,
        read::{
            BodyChunk, BodyChunks, BodyReader, Error, ReadBody, error_unknown_chunk_version,
            read_body_chunks,
        },
    };

    impl ReadBody for MediaBlockCameraCustom {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaBlockCameraCustom {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(6, Self::read_chunk_6)]
        }
    }

    impl MediaBlockCameraCustom {
        fn read_chunk_6(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(error_unknown_chunk_version(version));
            }

            let _keys = r.list(|r| {
                let _time = r.f32()?;
                let _interpolation = r.u32()?;
                let _anchor_rot = r.bool32()?;
                let _anchor = r.u32()?;
                let _anchor_vis = r.bool32()?;
                let _target = r.u32()?;
                let _position = r.vec3()?;
                let _pitch_yaw_roll = r.vec3()?;
                let _fov = r.f32()?;
                let _target_position = r.vec3()?;
                let _near_z = r.f32()?;

                let _position = r.vec3()?;
                let _pitch_yaw_roll = r.vec3()?;
                let _fov = r.f32()?;
                let _target_position = r.vec3()?;
                let _near_z = r.f32()?;

                let _position = r.vec3()?;
                let _pitch_yaw_roll = r.vec3()?;
                let _fov = r.f32()?;
                let _target_position = r.vec3()?;
                let _near_z = r.f32()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
