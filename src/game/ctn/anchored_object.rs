use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::read::{BodyChunksReader, BodyReader, ClassId, Error, ReadNode};

pub struct AnchoredObject(Inner);

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
    chunk_4: Chunk4,
    chunk_5: Chunk5,
}

struct Chunk2;

struct Chunk4;

struct Chunk5;

impl ClassId for AnchoredObject {
    const CLASS_ID: u32 = 0x03101000;
}

impl ReadNode for AnchoredObject {
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

                let chunk_2 = r.chunk(0x03101002, |r| {
                    let version = r.u32()?;

                    if version != 8 {
                        return Err(Error::new(format!("unknown chunk version: {version}")));
                    }

                    let _item_model_id = r.id()?;
                    let _item_model_collection = r.id()?;
                    let _item_model_author = r.id()?;
                    let _yaw_pitch_roll = r.vec3_f32()?;
                    let _block_unit_coord = r.vec3_u8()?;
                    let _anchor_tree_id = r.id_or_null()?;
                    let _absolute_position_in_map = r.vec3_f32()?;
                    let _waypoint_special_property = r.u32()?;
                    let flags = r.u16()?;
                    let _pivot_position = r.vec3_f32()?;
                    let _scale = r.f32()?;

                    if flags & 0x0004 != 0 {
                        todo!()
                    }

                    r.vec3_f32()?;
                    r.vec3_f32()?;

                    Ok(Chunk2)
                })?;

                let chunk_4 = r.skippable_chunk(0x03101004, |r| {
                    let version = r.u32()?;

                    if version != 0 {
                        return Err(Error::new(format!("unknown chunk version: {version}")));
                    }

                    r.u32()?;

                    Ok(Chunk4)
                })?;

                let chunk_5 = r.skippable_chunk(0x03101005, |r| {
                    r.u32()?;
                    r.u32()?;
                    r.u8()?;

                    Ok(Chunk5)
                })?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_2,
                    chunk_4,
                    chunk_5,
                })
            },
        };

        builder.try_build().map(Self)
    }
}
