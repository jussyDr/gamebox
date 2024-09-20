use std::io::Read;

use crate::{
    read::{
        readable::{BodyChunk, BodyChunks},
        IdStateMut, NodeStateMut, Reader,
    },
    Error,
};

use super::MediaClip;

pub enum Condition {
    None,
    RaceTimeLessThan,
    RaceTimeGreaterThan,
    AlreadyTriggered,
    SpeedLessThan,
    SpeedGreaterThan,
    NotAlreadyTriggered,
    MaxPlayCount,
    RandomOnce,
    Random,
}

impl Condition {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let condition = match r.u32()? {
            0 => Self::None,
            1 => Self::RaceTimeLessThan,
            2 => Self::RaceTimeGreaterThan,
            3 => Self::AlreadyTriggered,
            4 => Self::SpeedLessThan,
            5 => Self::SpeedGreaterThan,
            6 => Self::NotAlreadyTriggered,
            7 => Self::MaxPlayCount,
            8 => Self::RandomOnce,
            9 => Self::Random,
            _ => return Err(Error),
        };

        Ok(condition)
    }
}

/// A media clip group.
#[derive(Default)]
pub struct MediaClipGroup;

impl BodyChunks for MediaClipGroup {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(3, |n, r| Self::read_chunk_3(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaClipGroup {
    fn read_chunk_3(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<(), Error> {
        let _clips = r.versioned_list(|r| r.node::<MediaClip>())?;
        let _triggers = r.list(|r| {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            let _condition = Condition::read(r)?;
            let _condition_value = r.f32()?;
            let _coords = r.list(|r| r.vec3::<u32>())?;

            Ok(())
        })?;

        Ok(())
    }
}
