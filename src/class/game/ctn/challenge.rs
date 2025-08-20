use std::{io, sync::Arc};

use crate::{
    game::ctn::{Block, ChallengeParameters, CollectorList, FileRef},
    read::{self, BodyReader, Error, Read, ReadEnum, Result, read_body_chunks},
    write::{self, BodyWriter, Write},
};

pub struct Challenge {
    chunk_13: Chunk13,
    chunk_17: Chunk17,
    chunk_24: Chunk24,
    chunk_25: Chunk25,
    chunk_31: Chunk31,
}

struct Chunk13;

struct Chunk17;

struct Chunk24;

struct Chunk25;

struct Chunk31;

enum MapKind {
    InProgress,
}

impl ReadEnum for MapKind {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            6 => Ok(Self::InProgress),
            _ => Err(Error::Internal(
                "unknown variant index of enum MapKind".into(),
            )),
        }
    }
}

impl Read for Challenge {}

impl read::sealed::Read for Challenge {
    const CLASS_ID: u32 = 0x03043000;

    fn read_body(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_13: r.chunk(0x0304300d, |r| {
                    let _player_model_id = r.string_ref()?;
                    let _player_model_collection = r.string_ref()?;
                    let _player_model_author = r.string_ref()?;

                    Ok(Chunk13)
                })?,
                chunk_17: r.chunk(0x03043011, |r| {
                    let _block_stock = r.node_ref::<Arc<CollectorList>>()?;
                    let _challenge_parameters = r.node_ref::<Arc<ChallengeParameters>>()?;
                    let _map_kind = r.enum32::<MapKind>()?;

                    Ok(Chunk17)
                })?,
                chunk_24: r.chunk_skippable(0x03043018, |r| {
                    let _is_lap_race = r.bool32()?;
                    let _num_laps = r.u32()?;

                    Ok(Chunk24)
                })?,
                chunk_25: r.chunk_skippable(0x03043019, |r| {
                    let _texture_mod = FileRef::read(r)?;

                    Ok(Chunk25)
                })?,
                chunk_31: r.chunk(0x0304301f, |r| {
                    let _map_id = r.string_ref()?;
                    let _map_collection = r.string_ref()?;
                    let _map_author = r.string_ref()?;
                    let _map_name = r.string()?;
                    let _deco_id = r.string_ref()?;
                    let _deco_collection = r.string_ref()?;
                    let _deco_author = r.string_ref()?;
                    let _size = r.vec3_u32()?;
                    let _need_unlock = r.bool32()?;

                    if r.u32()? != 6 {
                        return Err(Error::Internal("unknown blocks version".into()));
                    }

                    let _blocks = r.list(Block::read)?;

                    Ok(Chunk31)
                })?,
            })
        })
    }
}

impl Write for Challenge {}

impl write::sealed::Write for Challenge {
    const CLASS_ID: u32 = 0x03043000;

    fn write_header(&self, w: &mut impl write::HeaderWriter) -> io::Result<()> {
        todo!()
    }

    fn write_body(&self, w: &mut impl BodyWriter) -> io::Result<()> {
        todo!()
    }
}
