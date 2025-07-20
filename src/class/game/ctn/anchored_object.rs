use std::{any::Any, cell::OnceCell, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    game::WaypointSpecialProperty,
    read::{BodyChunksReader, BodyReader, ClassId, Error, ReadNode},
};

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
    chunk_2: Chunk2<'a>,
    chunk_4: Chunk4,
    chunk_5: Chunk5,
}

struct Chunk2<'a> {
    item_model_id: &'a str,
}

struct Chunk4;

struct Chunk5;

impl AnchoredObject {
    pub fn item_model_id(&self) -> &str {
        self.0.borrow_chunks().chunk_2.item_model_id
    }
}

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

                let chunk_2 = r.chunk(0x03101002, Chunk2::read)?;
                let chunk_4 = r.skippable_chunk(0x03101004, Chunk4::read)?;
                let chunk_5 = r.skippable_chunk(0x03101005, Chunk5::read)?;
                r.end()?;

                Ok(Chunks {
                    chunk_2,
                    chunk_4,
                    chunk_5,
                })
            },
        };

        builder.try_build().map(Self)
    }
}

impl<'a> Chunk2<'a> {
    fn read(r: &mut BodyReader<'a, '_>) -> Result<Self, Error> {
        let version = r.u32()?;

        if version != 8 {
            return Err(Error::new(format!("unknown chunk version: {version}")));
        }

        let item_model_id = r.id()?;
        let _item_model_collection = r.id()?;
        let _item_model_author = r.id()?;
        let _yaw_pitch_roll = r.vec3_f32()?;
        let _block_unit_coord = r.vec3_u8()?;
        let _anchor_tree_id = r.id_or_null()?;
        let _absolute_position_in_map = r.vec3_f32()?;
        let _waypoint_special_property = r.node_ref_or_null::<WaypointSpecialProperty>()?;
        let flags = r.u16()?;
        let _pivot_position = r.vec3_f32()?;
        let _scale = r.f32()?;

        if flags & 0x0004 != 0 {
            todo!()
        }

        r.vec3_f32()?;
        r.vec3_f32()?;

        Ok(Self { item_model_id })
    }
}

impl Chunk4 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let version = r.u32()?;

        if version != 0 {
            return Err(Error::new(format!("unknown chunk version: {version}")));
        }

        r.u32()?;

        Ok(Self)
    }
}

impl Chunk5 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        r.u32()?;
        r.u32()?;
        r.u8()?;

        Ok(Self)
    }
}
