use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::read::{BodyChunksReader, BodyReader, Error, ReadNodeRef};

pub struct CollectorList(Inner);

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

impl ReadNodeRef for CollectorList {
    fn read_from_body(
        body_data: Arc<[u8]>,
        node_refs: Arc<[OnceCell<Box<dyn Any>>]>,
        body_data_offset: &mut usize,
    ) -> Result<Self, Error> {
        let builder = InnerTryBuilder {
            body_data,
            node_refs,
            chunks_builder: |body_data, node_refs| {
                let mut r = BodyChunksReader(BodyReader::new(
                    Arc::clone(body_data),
                    Arc::clone(node_refs),
                    body_data,
                    node_refs,
                    body_data_offset,
                ));

                let chunk_0 = r.chunk(0x0301b000, |r| {
                    let _collector_stock = r.list(|r| {
                        let _block_model = r.id()?;
                        let _block_model_collection = r.id()?;
                        let _block_model_author = r.id()?;
                        let _count = r.u32()?;

                        Ok(())
                    })?;

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
