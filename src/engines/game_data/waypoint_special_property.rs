//! Types used for reading [WaypointSpecialProperty] nodes.

use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    read::Reader,
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
};

/// Special waypoint property.
#[derive(Clone, Debug)]
pub enum WaypointSpecialProperty {
    /// A checkpoint.
    Checkpoint { group: u32 },
    /// A finish.
    Goal { order: u32 },
    /// A linked checkpoint.
    LinkedCheckpoint { group: u32 },
    /// A start.
    Spawn { order: u32 },
    /// A multilap.
    StartFinish { order: u32 },
    /// Custom waypoint.
    Custom { tag: String },
}

impl Class for WaypointSpecialProperty {
    const CLASS_ID: ClassId = ClassId::new(EngineId::GAME_DATA, 9);
}

impl Default for WaypointSpecialProperty {
    fn default() -> Self {
        Self::Checkpoint { group: 0 }
    }
}

impl<R: Read, I, N> ReadBody<R, I, N> for WaypointSpecialProperty {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        read_body_chunks(self, r)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for WaypointSpecialProperty {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x2e009000,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_2e009000(n, r)),
            },
            BodyChunkEntry {
                id: 0x2e009001,
                read_fn: BodyChunkReadFn::Skippable(|n, r| Self::read_chunk_2e009001(n, r)),
            },
        ]
        .into_iter()
    }
}

impl WaypointSpecialProperty {
    fn read_chunk_2e009000<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 2
        *self = match r.string()?.as_str() {
            "Checkpoint" => Self::Checkpoint { group: r.u32()? },
            "Goal" => Self::Goal { order: r.u32()? },
            "LinkedCheckpoint" => Self::LinkedCheckpoint { group: r.u32()? },
            "Spawn" => Self::Spawn { order: r.u32()? },
            "StartFinish" => Self::StartFinish { order: r.u32()? },
            tag => {
                r.u32()?;

                Self::Custom {
                    tag: tag.to_owned(),
                }
            }
        };

        Ok(())
    }

    fn read_chunk_2e009001<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0
        r.u32()?; // 0

        Ok(())
    }
}
