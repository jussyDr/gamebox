use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    game::ctn::FileRef,
    read::{BodyChunksReader, BodyReader, ClassId, Error, ReadNode},
};

pub struct BlockSkin(Inner);

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
    chunk_3: Chunk3,
}

struct Chunk2;

struct Chunk3;

impl ClassId for BlockSkin {
    const CLASS_ID: u32 = 0x03059000;
}

impl ReadNode for BlockSkin {
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

                let chunk_2 = r.chunk(0x03059002, Chunk2::read)?;
                let chunk_3 = r.chunk(0x03059003, Chunk3::read)?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_2,
                    chunk_3,
                })
            },
        };

        builder.try_build().map(Self)
    }
}

impl Chunk2 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let _text = r.string()?;
        let _file_ref = FileRef::read(r)?;
        let _parent_file_ref = FileRef::read(r)?;

        Ok(Self)
    }
}

impl Chunk3 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let version = r.u32()?;

        if version != 0 {
            return Err(Error::new(format!("unknown chunk version: {version}")));
        }

        let _foreground_file_ref = FileRef::read(r)?;

        Ok(Self)
    }
}
