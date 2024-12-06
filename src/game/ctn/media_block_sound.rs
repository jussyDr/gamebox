//! Media block sound.

use crate::Class;

/// Sound media block.
#[derive(Default)]
pub struct MediaBlockSound;

impl Class for MediaBlockSound {
    const CLASS_ID: u32 = 0x030a7000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::MediaBlockSound;

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
            let _sound = r.pack_desc()?;

            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let _keys = r.list(|r| {
                let _time = r.f32()?;
                let _volume = r.f32()?;
                let _pan = r.f32()?;
                let _position = r.vec3::<f32>()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
