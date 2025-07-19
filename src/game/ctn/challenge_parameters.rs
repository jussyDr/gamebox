use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    game::ctn::Ghost,
    read::{BodyChunksReader, BodyReader, Error, ReadNodeRef},
};

pub struct ChallengeParameters(Inner);

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
    chunk_4: Chunk4,
    chunk_8: Chunk8,
    chunk_10: Chunk10,
    chunk_13: Chunk13,
    chunk_14: Chunk14,
}

struct Chunk1;

struct Chunk4;

struct Chunk8;

struct Chunk10;

struct Chunk13;

struct Chunk14;

impl ReadNodeRef for ChallengeParameters {
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

                let chunk_1 = r.chunk(0x0305b001, |r| {
                    let _tip_1 = r.string()?;
                    let _tip_2 = r.string()?;
                    let _tip_3 = r.string()?;
                    let _tip_4 = r.string()?;

                    Ok(Chunk1)
                })?;

                let chunk_4 = r.chunk(0x0305b004, |r| {
                    let _bronze_time = r.u32()?;
                    let _silver_time = r.u32()?;
                    let _gold_time = r.u32()?;
                    let _author_time = r.u32()?;
                    r.u32()?;

                    Ok(Chunk4)
                })?;

                let chunk_8 = r.chunk(0x0305b008, |r| {
                    let _time_limit = r.u32()?;
                    let _author_score = r.u32()?;

                    Ok(Chunk8)
                })?;

                let chunk_10 = r.skippable_chunk(0x0305b00a, |r| {
                    let _tip = r.string()?;
                    let _bronze_time = r.u32()?;
                    let _silver_time = r.u32()?;
                    let _gold_time = r.u32()?;
                    let _author_time = r.u32()?;
                    let _time_limit = r.u32()?;
                    let _author_score = r.u32()?;

                    Ok(Chunk10)
                })?;

                let chunk_13 = r.chunk(0x0305b00d, |r| {
                    let _race_validate_ghost = r.node_ref_or_null::<Ghost>()?;

                    Ok(Chunk13)
                })?;

                let chunk_14 = r.skippable_chunk(0x0305b00e, |r| {
                    let _map_type = r.string()?;
                    let _map_style = r.string()?;
                    let _is_validated_for_script_modes = r.bool32()?;

                    Ok(Chunk14)
                })?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_1,
                    chunk_4,
                    chunk_8,
                    chunk_10,
                    chunk_13,
                    chunk_14,
                })
            },
        };

        builder.try_build().map(Self)
    }
}
