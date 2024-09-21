use std::io::Read;

use crate::{
    read::{
        readable::{BodyChunk, BodyChunks},
        Reader,
    },
    Error,
};

/// A sound media block.
pub struct MediaBlockSound;

impl BodyChunks for MediaBlockSound {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 2] = [
            (3, |n, r| Self::read_chunk_3(n, r), false),
            (4, |n, r| Self::read_chunk_4(n, r), false),
        ];

        chunks.into_iter()
    }
}

impl MediaBlockSound {
    fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 2 {
            return Err(Error);
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
            return Err(Error);
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
