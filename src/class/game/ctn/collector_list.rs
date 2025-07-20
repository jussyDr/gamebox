use std::{any::Any, cell::OnceCell, sync::Arc};

use ouroboros::self_referencing;

use crate::read::{BodyChunksReader, BodyReader, ClassId, Error, ReadNode};

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
    chunk_0: Chunk0<'a>,
}

struct Chunk0<'a> {
    collectors: Box<[Collector<'a>]>,
}

pub struct Collector<'a> {
    id: &'a str,
    count: u32,
}

impl CollectorList {
    pub fn collectors(&self) -> &[Collector] {
        &self.0.borrow_chunks().chunk_0.collectors
    }
}

impl Collector<'_> {
    pub fn id(&self) -> &str {
        self.id
    }

    pub fn count(&self) -> u32 {
        self.count
    }
}

impl ClassId for CollectorList {
    const CLASS_ID: u32 = 0x0301b000;
}

impl ReadNode for CollectorList {
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

                let chunk_0 = r.chunk(0x0301b000, Chunk0::read)?;

                r.end()?;

                Ok(Chunks { chunk_0 })
            },
        };

        builder.try_build().map(Self)
    }
}

impl<'a> Chunk0<'a> {
    fn read(r: &mut BodyReader<'a, '_>) -> Result<Self, Error> {
        let collectors = r.list(|r| {
            let id = r.id()?;
            let _block_model_collection = r.id()?;
            let _block_model_author = r.id()?;
            let count = r.u32()?;

            Ok(Collector { id, count })
        })?;

        Ok(Self { collectors })
    }
}
