use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

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
    delme: PhantomData<&'a ()>,
    chunk_0: Chunk0,
}

struct Chunk0;

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

                let chunk_0 = r.chunk(0x030a5000, |r| {
                    let _effect = r.node_ref::<EffectSimi>()?;
                    let _image = FileRef::read(r)?;

                    Ok(Chunk0)
                })?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_0,
                })
            },
        };

        builder.try_build().map(Self)
    }
}
