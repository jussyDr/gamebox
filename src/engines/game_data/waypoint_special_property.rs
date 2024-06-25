use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::Deserializer,
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
};

/// Special waypoint property.
#[derive(Clone, Debug)]
pub enum WaypointSpecialProperty {
    Checkpoint { group: u32 },
    Goal { order: u32 },
    LinkedCheckpoint { group: u32 },
    Spawn { order: u32 },
    StartFinish { order: u32 },
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
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I, N> BodyChunks<R, I, N> for WaypointSpecialProperty {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x2e009000,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_2e009000(n, d)),
            },
            BodyChunkEntry {
                id: 0x2e009001,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_2e009001(n, d)),
            },
        ]
        .into_iter()
    }
}

impl WaypointSpecialProperty {
    fn read_chunk_2e009000<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        *self = match d.string()?.as_str() {
            "Checkpoint" => Self::Checkpoint { group: d.u32()? },
            "Goal" => Self::Goal { order: d.u32()? },
            "LinkedCheckpoint" => Self::LinkedCheckpoint { group: d.u32()? },
            "Spawn" => Self::Spawn { order: d.u32()? },
            "StartFinish" => Self::StartFinish { order: d.u32()? },
            tag => {
                d.u32()?;

                Self::Custom {
                    tag: tag.to_owned(),
                }
            }
        };

        Ok(())
    }

    fn read_chunk_2e009001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}
