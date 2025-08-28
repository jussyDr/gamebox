use std::sync::Arc;

use crate::{
    control::EffectSimi,
    game::ctn::FileRef,
    read::{BodyReader, ReadNode, Result, read_body_chunks},
};

pub struct Image {
    chunk_0: Chunk0,
}

struct Chunk0;

impl ReadNode for Image {
    const CLASS_ID: u32 = 0x030a5000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            let chunk_0 = r.chunk(0x030a5000, |r| {
                let _effect = r.node_ref::<Arc<EffectSimi>>()?;
                let _image = FileRef::read(r)?;

                Ok(Chunk0)
            })?;

            Ok(Self { chunk_0 })
        })
    }
}
