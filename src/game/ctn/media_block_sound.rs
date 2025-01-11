//! Media block sound.

use bytemuck::cast;
use ordered_float::OrderedFloat;

use crate::{Class, FileRef, OrderedVec3, Vec3};

/// Sound media block.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaBlockSound {
    sound: FileRef,
    keys: Vec<Key>,
}

impl Class for MediaBlockSound {
    const CLASS_ID: u32 = 0x030a7000;
}

impl MediaBlockSound {
    /// Sound.
    pub const fn sound(&self) -> &FileRef {
        &self.sound
    }

    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Sound media block key.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Key {
    time: OrderedFloat<f32>,
    volume: OrderedFloat<f32>,
    pan: OrderedFloat<f32>,
    position: OrderedVec3,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time.0
    }

    /// Volume.
    pub const fn volume(&self) -> f32 {
        self.volume.0
    }

    /// Pan.
    pub const fn pan(&self) -> f32 {
        self.pan.0
    }

    /// Position.
    pub fn position(&self) -> Vec3 {
        cast(self.position)
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

    use super::{Key, MediaBlockSound};

    impl ReadBody for MediaBlockSound {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaBlockSound {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(3, Self::read_chunk_3),
                BodyChunk::normal(4, Self::read_chunk_4),
            ]
            .into_iter()
        }
    }

    impl MediaBlockSound {
        fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            let _play_count = r.u32()?;
            let _is_looping = r.bool()?;
            let _is_music = r.bool()?;
            let _stop_with_clip = r.bool()?;
            let _audio_to_speech = r.bool()?;
            let _audio_to_speech_target = r.u32()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.sound = r.file_ref()?;

            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            self.keys = r.list(|r| {
                let time = r.f32()?;
                let volume = r.f32()?;
                let pan = r.f32()?;
                let position = r.vec3_ordered()?;

                Ok(Key {
                    time: OrderedFloat(time),
                    volume: OrderedFloat(volume),
                    pan: OrderedFloat(pan),
                    position,
                })
            })?;

            Ok(())
        }
    }
}
