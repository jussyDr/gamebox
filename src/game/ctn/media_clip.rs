//! Media clip.

use std::sync::Arc;

use crate::Class;

use super::MediaTrack;

/// A media clip.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct MediaClip {
    tracks: Vec<Arc<MediaTrack>>,
    name: String,
    stop_when_leave: bool,
    stop_when_respawn: bool,
}

impl Class for MediaClip {
    const CLASS_ID: u32 = 0x03079000;
}

impl MediaClip {
    /// Tracks.
    pub const fn tracks(&self) -> &Vec<Arc<MediaTrack>> {
        &self.tracks
    }

    /// Name.
    pub const fn name(&self) -> &String {
        &self.name
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::MediaClip;

    impl ReadBody for MediaClip {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaClip {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(13, Self::read_chunk_13),
                BodyChunk::skippable(14, Self::read_chunk_14),
            ]
            .into_iter()
        }
    }

    impl MediaClip {
        fn read_chunk_13(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 0 | 1) {
                return Err(Error::chunk_version(version));
            }

            self.tracks = r.list_with_version(|r| r.internal_node_ref())?;
            self.name = r.string()?;
            self.stop_when_leave = r.bool()?;
            r.bool()?;
            self.stop_when_respawn = r.bool()?;
            r.string_or_empty()?;
            r.f32()?;
            let _local_player_clip_ent_index = r.u32()?;

            Ok(())
        }

        fn read_chunk_14<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
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

    use super::MediaClip;

    impl WriteBody for MediaClip {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for MediaClip {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [
                BodyChunk::normal(13, Self::write_chunk_13),
                BodyChunk::skippable(14, |s, w| Self::write_chunk_14(s, w)),
            ]
            .into_iter()
        }
    }

    impl MediaClip {
        fn write_chunk_13(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            w.u32(1)?;
            w.list_with_version(&self.tracks, |w, track| w.internal_node_ref(track))?;
            w.string(&self.name)?;
            w.bool(self.stop_when_leave)?;
            w.bool(false)?;
            w.bool(self.stop_when_respawn)?;
            w.string_or_empty(None)?;
            w.f32(0.2)?;
            w.u32(0)?;

            Ok(())
        }

        fn write_chunk_14<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(1)?;
            w.u32(0)?;

            Ok(())
        }
    }
}
