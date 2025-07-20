use std::{any::Any, cell::OnceCell, sync::Arc};

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
    chunk_1: Chunk1<'a>,
    chunk_2: Chunk2,
}

struct Chunk1<'a> {
    text: &'a str,
    effect: &'a EffectSimi,
}

struct Chunk2;

impl MediaBlockText {
    pub fn text(&self) -> &str {
        self.0.borrow_chunks().chunk_1.text
    }

    pub fn effect(&self) -> &EffectSimi {
        self.0.borrow_chunks().chunk_1.effect
    }
}

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

                let chunk_1 = r.chunk(0x030a8001, Chunk1::read)?;
                let chunk_2 = r.chunk(0x030a8002, Chunk2::read)?;

                r.end()?;

                Ok(Chunks { chunk_1, chunk_2 })
            },
        };

        builder.try_build().map(Self)
    }
}

impl<'a> Chunk1<'a> {
    fn read(r: &mut BodyReader<'a, '_>) -> Result<Self, Error> {
        let text = r.string()?;
        let effect = r.node_ref()?;

        Ok(Self { text, effect })
    }
}

impl Chunk2 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let _color = r.vec3_f32()?;

        Ok(Self)
    }
}
