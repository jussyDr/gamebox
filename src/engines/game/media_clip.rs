use std::io::Read;

use crate::{
    read::{
        readable::{BodyChunk, BodyChunks},
        IdStateMut, NodeStateMut, Reader,
    },
    Error,
};

use super::MediaTrack;

/// A media clip.
#[derive(Default)]
pub struct MediaClip;

impl BodyChunks for MediaClip {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 2] = [
            (13, |n, r| Self::read_chunk_13(n, r), false),
            (14, |n, r| Self::read_chunk_14(n, r), true),
        ];

        chunks.into_iter()
    }
}

impl MediaClip {
    fn read_chunk_13(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 1 {
            return Err(Error);
        }

        let _tracks = r.versioned_list(|r| r.node::<MediaTrack>())?;
        let _name = r.string()?;
        let _stop_when_leave = r.bool()?;
        r.bool()?;
        let _stop_when_respawn = r.bool()?;
        r.string()?;
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
