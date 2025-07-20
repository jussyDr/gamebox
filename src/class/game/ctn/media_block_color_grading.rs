use std::{any::Any, cell::OnceCell, sync::Arc};

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
    chunk_0: Chunk0<'a>,
    chunk_1: Chunk1,
}

struct Chunk0<'a> {
    image: FileRef<'a>,
}

struct Chunk1 {
    keys: Box<[Key]>,
}

pub struct Key;

impl MediaBlockColorGrading {
    pub fn image(&self) -> &FileRef {
        &self.0.borrow_chunks().chunk_0.image
    }

    pub fn keys(&self) -> &[Key] {
        &self.0.borrow_chunks().chunk_1.keys
    }
}

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

                let chunk_0 = r.chunk(0x03186000, Chunk0::read)?;
                let chunk_1 = r.chunk(0x03186001, Chunk1::read)?;

                r.end()?;

                Ok(Chunks { chunk_0, chunk_1 })
            },
        };

        builder.try_build().map(Self)
    }
}

impl<'a> Chunk0<'a> {
    fn read(r: &mut BodyReader<'a, '_>) -> Result<Self, Error> {
        let image = FileRef::read(r)?.unwrap();

        Ok(Self { image })
    }
}

impl Chunk1 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let keys = r.list(|r| {
            let _time = r.f32()?;
            let _intensity = r.f32()?;

            Ok(Key)
        })?;

        Ok(Self { keys })
    }
}
