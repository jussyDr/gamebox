//! Media clip.

use std::sync::Arc;

use crate::{ClassId, class::game::ctn::media_track::MediaTrack};

/// Media clip.
#[derive(Default)]
pub struct MediaClip {
    tracks: Vec<Arc<MediaTrack>>,
    name: String,
}

impl MediaClip {
    /// Tracks.
    pub fn tracks(&self) -> &Vec<Arc<MediaTrack>> {
        &self.tracks
    }

    /// Name.
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl ClassId for MediaClip {
    const CLASS_ID: u32 = 0x03079000;
}

mod read {
    use crate::{
        class::game::ctn::{media_clip::MediaClip, media_track::MediaTrack},
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
    };

    impl ReadBody for MediaClip {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaClip {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(13, Self::read_chunk_13)]
        }
    }

    impl MediaClip {
        fn read_chunk_13(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            self.tracks = r.list_with_version(|r| r.internal_node_ref::<MediaTrack>())?;
            self.name = r.string()?;
            let _stop_when_leave = r.bool32()?;
            r.bool32()?;
            let _step_when_respawn = r.bool32()?;
            r.string()?;
            r.f32()?;
            let _local_player_clip_ent_index = r.u32()?;

            Ok(())
        }
    }
}
