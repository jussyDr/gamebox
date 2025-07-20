use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    game::ctn::MediaBlock,
    read::{BodyChunksReader, BodyReader, ClassId, Error, ReadNode},
};

pub struct MediaTrack(Inner);

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
    chunk_1: Chunk1,
    chunk_5: Chunk5,
}

struct Chunk1;

struct Chunk5;

impl ClassId for MediaTrack {
    const CLASS_ID: u32 = 0x03078000;
}

impl ReadNode for MediaTrack {
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

                let chunk_1 = r.chunk(0x03078001, Chunk1::read)?;
                let chunk_5 = r.chunk(0x03078005, Chunk5::read)?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_1,
                    chunk_5,
                })
            },
        };

        builder.try_build().map(Self)
    }
}

impl Chunk1 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let _name = r.string()?;
        let _blocks = r.list_with_version(|r| r.node_ref_generic::<MediaBlock>())?;
        r.u32()?;

        Ok(Self)
    }
}

impl Chunk5 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let version = r.u32()?;

        if version != 1 {
            return Err(Error::new(format!("unknown chunk version: {version}")));
        }

        let _is_keep_playing = r.bool32()?;
        let _is_read_only = r.bool32()?;
        let _is_cycling = r.bool32()?;
        let _repeating_segment_start = r.f32()?;
        let _repeating_segment_end = r.f32()?;

        Ok(Self)
    }
}
