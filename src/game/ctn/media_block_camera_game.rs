//! Media block camera game.

use ordered_float::OrderedFloat;

use crate::Class;

/// Media block camera game.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockCameraGame {
    start_time: OrderedFloat<f32>,
    end_time: OrderedFloat<f32>,
}

impl Class for MediaBlockCameraGame {
    const CLASS_ID: u32 = 0x03084000;
}

impl MediaBlockCameraGame {
    /// Start time.
    pub const fn start_time(&self) -> f32 {
        self.start_time.0
    }

    /// End time.
    pub const fn end_time(&self) -> f32 {
        self.end_time.0
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

            self.start_time = OrderedFloat(r.f32()?);
            self.end_time = OrderedFloat(r.f32()?);
            let _game_cam = r.u32()?;
            let _clip_ent_id = r.u32()?;
            let _cam_position = r.vec3()?;
            let _cam_rotation = r.yaw_pitch_roll()?;
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

mod write {
    use std::io::Write;

    use crate::write::{
        writable::{write_body_chunks, WriteBody},
        writer::{IdStateMut, NodeStateMut},
        BodyChunk, BodyChunks, Error, Writer,
    };

    use super::MediaBlockCameraGame;

    impl WriteBody for MediaBlockCameraGame {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for MediaBlockCameraGame {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
