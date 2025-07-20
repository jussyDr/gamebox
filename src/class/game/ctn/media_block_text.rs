use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    class::control::EffectSimi,
    read::{BodyChunksReader, BodyReader, ClassId, Error},
};

pub struct MediaBlockText(Inner);

#[self_referencing]
struct Inner {
    body_data: Arc<[u8]>,
    node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
    #[borrows(body_data, node_refs)]
    #[covariant]
    chunks: Chunks<'this>,
}

struct Chunks<'a> {
    delme: PhantomData<&'a ()>,
    chunk_1: Chunk1,
    chunk_2: Chunk2,
}

struct Chunk1;

struct Chunk2;

impl ClassId for MediaBlockText {
    const CLASS_ID: u32 = 0x030a8000;
}

impl MediaBlockText {
    pub fn read(
        body_data: Arc<[u8]>,
        body_data_offset: &mut usize,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
        seen_id: &mut bool,
        ids: &mut Vec<(usize, usize)>,
    ) -> Result<Self, Error> {
        let builder = InnerTryBuilder {
            body_data,
            node_refs,
            chunks_builder: |body_data, node_refs| {
                let mut br = BodyReader::new(body_data, body_data_offset, node_refs, seen_id, ids);
                let mut r = BodyChunksReader(&mut br);

                let chunk_1 = r.chunk(0x030a8001, |r| {
                    let _text = r.string()?;
                    let _effect = r.node_ref::<EffectSimi>()?;

                    Ok(Chunk1)
                })?;

                let chunk_2 = r.chunk(0x030a8002, |r| {
                    let _color = r.vec3_f32()?;

                    Ok(Chunk2)
                })?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_1,
                    chunk_2,
                })
            },
        };

        builder.try_build().map(Self)
    }
}
