use std::sync::Arc;

use crate::{
    game::ctn::MediaClip,
    read::{BodyReader, ReadEnum, ReadNode, Result, read_body_chunks},
};

pub struct MediaClipGroup {
    chunk_3: Chunk3,
}

struct Chunk3;

enum Condition {
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

impl ReadEnum for Condition {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::None),
            1 => Ok(Self::RaceTimeLessThan),
            2 => Ok(Self::RaceTimeGreaterThan),
            3 => Ok(Self::AlreadyTriggered),
            4 => Ok(Self::SpeedLessThan),
            5 => Ok(Self::SpeedGreaterThan),
            6 => Ok(Self::NotAlreadyTriggered),
            7 => Ok(Self::MaxPlayCount),
            8 => Ok(Self::RandomOnce),
            9 => Ok(Self::Random),
            _ => todo!(),
        }
    }
}

impl ReadNode for MediaClipGroup {
    const CLASS_ID: u32 = 0x0307a000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_3: r.chunk(0x0307a003, |r| {
                    let _clips = r.list_versioned(|r| r.node_ref::<Arc<MediaClip>>())?;
                    let _triggers = r.list(|r| {
                        r.u32()?;
                        r.u32()?;
                        r.u32()?;
                        r.u32()?;
                        let _condition = r.enum32::<Condition>()?;
                        let _condition_value = r.f32()?;
                        let _coords = r.list(|r| r.vec3_u32())?;

                        Ok(())
                    })?;

                    Ok(Chunk3)
                })?,
            })
        })
    }
}
