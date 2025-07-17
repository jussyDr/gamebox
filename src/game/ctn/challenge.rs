use std::any::Any;

use ouroboros::self_referencing;

use crate::{
    game::ctn::Block,
    read::{BodyChunksReader, BodyReader, Error, IdRefs},
};

pub struct Challenge {
    body: Body,
}

#[self_referencing]
struct Body {
    body_data: Box<[u8]>,
    node_refs: Box<[Option<Box<dyn Any>>]>,
    #[borrows(body_data, node_refs)]
    #[covariant]
    chunks: BodyChunks<'this>,
}

pub struct BodyChunks<'a> {
    chunk_31: Chunk31<'a>,
}

struct Chunk31<'a> {
    blocks: Vec<Block<'a>>,
}

impl Challenge {
    pub fn blocks(&self) -> &[Block] {
        &self.body.borrow_chunks().chunk_31.blocks
    }
}

impl Challenge {
    pub const CLASS_ID: u32 = 0x03043000;
}

impl Challenge {
    pub fn read_from_header_and_body(
        header_data: Box<[u8]>,
        body_data: Box<[u8]>,
        node_refs: Box<[Option<Box<dyn Any>>]>,
    ) -> Result<Challenge, Error> {
        let body_builder = BodyTryBuilder {
            body_data,
            node_refs,
            chunks_builder: |body_data, node_refs| {
                let mut id_refs = IdRefs::new();
                let r = BodyReader::new(body_data, &mut id_refs, node_refs);
                let mut r = BodyChunksReader::new(r);

                Self::read_body_chunks(&mut r)?;

                todo!()
            },
        };

        let body = body_builder.try_build()?;

        Ok(Challenge { body })
    }

    pub fn read_body_chunks<'a>(r: &mut BodyChunksReader<'a>) -> Result<BodyChunks<'a>, Error> {
        let chunk_31 = r.chunk(0x0304301f, |r| {
            let _map_id = r.id()?;
            let _map_collection = r.id()?;
            let _map_author = r.id()?;
            let _map_name = r.string()?;
            let _decoration_id = r.id()?;
            let _decoration_collection = r.id()?;
            let _decoration_author = r.id()?;
            let _size = r.vec3_u32()?;
            let _need_unlock = r.bool32()?;
            let blocks_version = r.u32()?;

            if blocks_version != 6 {
                return Err(Error::unknown_version("blocks", blocks_version));
            }

            let blocks = r.list(Block::read_from_body)?;

            Ok(Chunk31 { blocks })
        })?;

        Ok(BodyChunks { chunk_31 })
    }
}
