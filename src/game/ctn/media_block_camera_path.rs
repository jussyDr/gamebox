//! Media block camera path.

use crate::Class;

/// Camera path media block.
#[derive(Default)]
pub struct MediaBlockCameraPath {
    keys: Vec<Key>,
}

impl Class for MediaBlockCameraPath {
    const CLASS_ID: u32 = 0x030a1000;
}

/// Camera path media block key.
pub struct Key {
    time: f32,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Key, MediaBlockCameraPath};

    impl ReadBody for MediaBlockCameraPath {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockCameraPath {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(3, Self::read_chunk_3)].into_iter()
        }
    }

    impl MediaBlockCameraPath {
        fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 5 {
                return Err(Error::chunk_version(version));
            }

            self.keys = r.list(|r| {
                let time = r.f32()?;
                let _position = r.vec3()?;
                let _rotation = r.pitch_yaw_roll()?;
                let _fov = r.f32()?;
                let _near_z = r.f32()?;
                let _anchor_rot = r.bool()?;
                let _anchor = r.u32()?;
                let _anchor_vis = r.bool()?;
                let _target = r.u32()?;
                let _target_position = r.vec3()?;
                let _weight = r.f32()?;
                r.quat()?;
                r.u32()?;
                r.u32()?;

                Ok(Key { time })
            })?;

            Ok(())
        }
    }
}
