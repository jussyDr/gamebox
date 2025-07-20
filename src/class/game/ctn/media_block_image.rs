use std::{any::Any, cell::OnceCell, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    class::control::EffectSimi,
    game::ctn::FileRef,
    read::{BodyChunksReader, BodyReader, ClassId, Error},
};

pub struct MediaBlockImage(Inner);

#[self_referencing]
struct Inner {
    body_data: Arc<[u8]>,
    node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
    #[borrows(body_data, node_refs)]
    #[covariant]
    chunks: Chunks<'this>,
}

struct Chunks<'a> {
    chunk_0: Chunk0<'a>,
}

struct Chunk0<'a> {
    effect: &'a EffectSimi,
    image: FileRef<'a>,
}

impl MediaBlockImage {
    pub fn effect(&self) -> &EffectSimi {
        self.0.borrow_chunks().chunk_0.effect
    }

    pub fn image(&self) -> &FileRef {
        &self.0.borrow_chunks().chunk_0.image
    }
}

impl ClassId for MediaBlockImage {
    const CLASS_ID: u32 = 0x030a5000;
}

impl MediaBlockImage {
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

                let chunk_0 = r.chunk(0x030a5000, Chunk0::read)?;

                r.end()?;

                Ok(Chunks { chunk_0 })
            },
        };

        builder.try_build().map(Self)
    }
}

impl<'a> Chunk0<'a> {
    fn read(r: &mut BodyReader<'a, '_>) -> Result<Self, Error> {
        let effect = r.node_ref()?;
        let image = FileRef::read(r)?.unwrap();

        Ok(Self { effect, image })
    }
}
