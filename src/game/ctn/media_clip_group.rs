//! Media clip group.

use std::sync::Arc;

use crate::{Class, Nat3};

use super::MediaClip;

/// A media clip group.
#[derive(PartialEq, Eq, Hash, Default)]
pub struct MediaClipGroup {
    clips: Vec<ClipTrigger>,
}

impl Class for MediaClipGroup {
    const CLASS_ID: u32 = 0x0307a000;
}

impl MediaClipGroup {
    /// Clips.
    pub const fn clips(&self) -> &Vec<ClipTrigger> {
        &self.clips
    }
}

/// Clip trigger.
#[derive(PartialEq, Eq, Hash)]
pub struct ClipTrigger {
    clip: Arc<MediaClip>,
    coords: Vec<Nat3>,
}

impl ClipTrigger {
    /// Clip.
    pub const fn clip(&self) -> &Arc<MediaClip> {
        &self.clip
    }

    /// Coordinates.
    pub const fn coords(&self) -> &Vec<Nat3> {
        &self.coords
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::MediaClip,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::{ClipTrigger, MediaClipGroup};

    impl ReadBody for MediaClipGroup {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaClipGroup {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(3, Self::read_chunk_3)].into_iter()
        }
    }

    impl MediaClipGroup {
        fn read_chunk_3(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let clips = r.list_with_version(|r| r.internal_node_ref::<MediaClip>())?;
            let num_triggers = r.u32()?;

            self.clips = vec![];

            for trigger_index in 0..num_triggers {
                let clip = clips[trigger_index as usize].clone();

                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                let _condition = r.u32()?;
                let _condition_value = r.f32()?;
                let coords = r.list_pod()?;

                self.clips.push(ClipTrigger { clip, coords });
            }

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

    use super::MediaClipGroup;

    impl WriteBody for MediaClipGroup {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for MediaClipGroup {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
