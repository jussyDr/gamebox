//! Media clip group.

use std::sync::Arc;

use ordered_float::OrderedFloat;

use crate::{Class, Nat3};

use super::MediaClip;

/// A media clip group.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
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
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ClipTrigger {
    clip: Arc<MediaClip>,
    condition: Option<Condition>,
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

/// Clip trigger condition.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Condition {
    /// Race time less than.
    RaceTimeLessThan(OrderedFloat<f32>),
    /// Race time greater than.
    RaceTimeGreaterThan(OrderedFloat<f32>),
    /// Already triggered.
    AlreadyTriggered(OrderedFloat<f32>),
    /// Speed less than.
    SpeedLessThan(OrderedFloat<f32>),
    /// Speed greater than.
    SpeedGreaterThan(OrderedFloat<f32>),
    /// Not already triggered.
    NotAlreadyTriggered(OrderedFloat<f32>),
    /// Max play count.
    MaxPlayCount(OrderedFloat<f32>),
    /// Random once.
    RandomOnce(OrderedFloat<f32>),
    /// Random.
    Random(OrderedFloat<f32>),
}

mod read {
    use std::io::{Read, Seek};

    use ordered_float::OrderedFloat;

    use crate::{
        game::ctn::MediaClip,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::{ClipTrigger, Condition, MediaClipGroup};

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
                let condition = r.u32()?;
                let condition_value = r.f32()?;
                let coords = r.list_pod()?;

                let condition = match condition {
                    0 => None,
                    1 => Some(Condition::RaceTimeLessThan(OrderedFloat(condition_value))),
                    2 => Some(Condition::RaceTimeGreaterThan(OrderedFloat(
                        condition_value,
                    ))),
                    3 => Some(Condition::AlreadyTriggered(OrderedFloat(condition_value))),
                    4 => Some(Condition::SpeedLessThan(OrderedFloat(condition_value))),
                    5 => Some(Condition::SpeedGreaterThan(OrderedFloat(condition_value))),
                    6 => Some(Condition::NotAlreadyTriggered(OrderedFloat(
                        condition_value,
                    ))),
                    7 => Some(Condition::MaxPlayCount(OrderedFloat(condition_value))),
                    8 => Some(Condition::RandomOnce(OrderedFloat(condition_value))),
                    9 => Some(Condition::Random(OrderedFloat(condition_value))),
                    _ => todo!(),
                };

                self.clips.push(ClipTrigger {
                    clip,
                    condition,
                    coords,
                });
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

    use super::{Condition, MediaClipGroup};

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
            [BodyChunk::normal(3, Self::write_chunk_3)].into_iter()
        }
    }

    impl MediaClipGroup {
        fn write_chunk_3(
            &self,
            w: &mut Writer<impl Write, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            w.list_with_version(&self.clips, |w, clip_trigger| {
                w.internal_node_ref(&clip_trigger.clip)
            })?;
            w.list(&self.clips, |w, clip_trigger| {
                w.u32(0xffffffff)?;
                w.u32(0xffffffff)?;
                w.u32(0xffffffff)?;
                w.u32(0)?;

                let (condition, condition_value) = match clip_trigger.condition {
                    None => (0, 0.0),
                    Some(Condition::RaceTimeLessThan(condition_value)) => (1, condition_value.0),
                    Some(Condition::RaceTimeGreaterThan(condition_value)) => (2, condition_value.0),
                    Some(Condition::AlreadyTriggered(condition_value)) => (3, condition_value.0),
                    Some(Condition::SpeedLessThan(condition_value)) => (4, condition_value.0),
                    Some(Condition::SpeedGreaterThan(condition_value)) => (5, condition_value.0),
                    Some(Condition::NotAlreadyTriggered(condition_value)) => (6, condition_value.0),
                    Some(Condition::MaxPlayCount(condition_value)) => (7, condition_value.0),
                    Some(Condition::RandomOnce(condition_value)) => (8, condition_value.0),
                    Some(Condition::Random(condition_value)) => (9, condition_value.0),
                };

                w.u32(condition)?;
                w.f32(condition_value)?;
                w.list_pod(&clip_trigger.coords)?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
