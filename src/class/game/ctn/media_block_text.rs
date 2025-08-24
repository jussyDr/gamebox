use std::sync::Arc;

use crate::{
    control::EffectSimi,
    read::{BodyReader, ReadNode, Result, read_body_chunks},
};

pub struct MediaBlockText {
    chunk_1: Chunk1,
    chunk_2: Chunk2,
}

struct Chunk1;

struct Chunk2;

impl ReadNode for MediaBlockText {
    const CLASS_ID: u32 = 0x030a8000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_1: r.chunk(0x030a8001, |r| {
                    let _text = r.string()?;
                    let _effect = r.node_ref::<Arc<EffectSimi>>()?;

                    Ok(Chunk1)
                })?,
                chunk_2: r.chunk(0x030a8002, |r| {
                    let _color = r.rgb_f32()?;

                    Ok(Chunk2)
                })?,
            })
        })
    }
}
