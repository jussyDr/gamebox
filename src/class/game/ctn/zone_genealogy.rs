use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::read::{BodyChunksReader, BodyReader, ClassId, Error, ReadNode};

pub struct ZoneGenealogy(Inner);

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
    chunk_2: Chunk2,
}

struct Chunk2;

impl ClassId for ZoneGenealogy {
    const CLASS_ID: u32 = 0x0311d000;
}

impl ReadNode for ZoneGenealogy {
    fn read_from_body(
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

                let chunk_2 = r.chunk(0x0311d002, Chunk2::read)?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_2,
                })
            },
        };

        builder.try_build().map(Self)
    }
}

impl Chunk2 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let _zone_ids = r.list(|r| r.id())?;
        let _current_index = r.u32()?;
        let _dir = r.u32()?;
        let _current_zone_id = r.id()?;

        Ok(Self)
    }
}
