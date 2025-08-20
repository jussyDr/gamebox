use std::sync::Arc;

use crate::{
    game::ctn::Ghost,
    read::{BodyReader, ReadNode, Result, read_body_chunks},
};

pub struct ChallengeParameters {
    chunk_1: Chunk1,
    chunk_4: Chunk4,
    chunk_8: Chunk8,
    chunk_10: Chunk10,
    chunk_13: Chunk13,
    chunk_14: Chunk14,
}

struct Chunk1;

struct Chunk4;

struct Chunk8;

struct Chunk10;

struct Chunk13;

struct Chunk14;

impl ReadNode for ChallengeParameters {
    const CLASS_ID: u32 = 0x0305b000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_1: r.chunk(0x0305b001, |r| {
                    let _tip_1 = r.string()?;
                    let _tip_2 = r.string()?;
                    let _tip_3 = r.string()?;
                    let _tip_4 = r.string()?;

                    Ok(Chunk1)
                })?,
                chunk_4: r.chunk(0x0305b004, |r| {
                    let _bronze_time = r.u32()?;
                    let _silver_time = r.u32()?;
                    let _gold_time = r.u32()?;
                    let _author_time = r.u32()?;
                    r.u32()?;

                    Ok(Chunk4)
                })?,
                chunk_8: r.chunk(0x0305b008, |r| {
                    let _time_limit = r.u32()?;
                    let _author_score = r.u32()?;

                    Ok(Chunk8)
                })?,
                chunk_10: r.chunk_skippable(0x0305b00a, |r| {
                    let _tip = r.string()?;
                    let _bronze_time = r.u32()?;
                    let _silver_time = r.u32()?;
                    let _gold_time = r.u32()?;
                    let _author_time = r.u32()?;
                    let _time_limit = r.u32()?;
                    let _author_score = r.u32()?;

                    Ok(Chunk10)
                })?,
                chunk_13: r.chunk(0x0305b00d, |r| {
                    let _race_validate_ghost = r.node_ref::<Arc<Ghost>>()?;

                    Ok(Chunk13)
                })?,
                chunk_14: r.chunk_skippable(0x0305b00e, |r| {
                    let _map_type = r.string()?;
                    let _map_style = r.string()?;
                    let _is_validated_for_script_modes = r.bool32()?;

                    Ok(Chunk14)
                })?,
            })
        })
    }
}
