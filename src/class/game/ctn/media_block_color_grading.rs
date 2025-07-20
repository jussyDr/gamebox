use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    game::ctn::FileRef,
    read::{BodyChunksReader, BodyReader, ClassId, Error},
};

pub struct MediaBlockColorGrading(Inner);

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
    chunk_1: Chunk1,
}

struct Chunk0;

struct Chunk1;

impl ClassId for MediaBlockColorGrading {
    const CLASS_ID: u32 = 0x03186000;
}

impl MediaBlockColorGrading {
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

                let chunk_0 = r.chunk(0x03186000, |r| {
                    let _image = FileRef::read(r)?;

                    Ok(Chunk0)
                })?;

                let chunk_1 = r.chunk(0x03186001, |r| {
                    let _keys = r.list(|r| {
                        let _time = r.f32()?;
                        let _intensity = r.f32()?;

                        Ok(())
                    })?;

                    Ok(Chunk1)
                })?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_0,
                    chunk_1,
                })
            },
        };

        builder.try_build().map(Self)
    }
}
