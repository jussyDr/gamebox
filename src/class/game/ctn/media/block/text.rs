use std::sync::Arc;

use crate::{
    Rgb,
    control::EffectSimi,
    read::{BodyReader, ReadNode, Result, read_body_chunks},
};

pub struct Text {
    chunk_1: Chunk1,
    chunk_2: Chunk2,
}

struct Chunk1 {
    text: String,
    effect: Arc<EffectSimi>,
}

struct Chunk2 {
    color: Rgb,
}

impl Text {
    /// Text.
    pub fn text(&self) -> &str {
        &self.chunk_1.text
    }

    pub fn effect(&self) -> &EffectSimi {
        &self.chunk_1.effect
    }

    /// Text color.
    pub fn color(&self) -> Rgb {
        self.chunk_2.color
    }
}

impl ReadNode for Text {
    const CLASS_ID: u32 = 0x030a8000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_1: r.chunk(0x030a8001, |r| {
                    let text = r.string()?;
                    let effect = r.node_ref()?;

                    Ok(Chunk1 { text, effect })
                })?,
                chunk_2: r.chunk(0x030a8002, |r| {
                    let color = r.rgb()?;

                    Ok(Chunk2 { color })
                })?,
            })
        })
    }
}
