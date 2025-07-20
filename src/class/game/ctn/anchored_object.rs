use std::{any::Any, cell::OnceCell, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    F32Vec3, U8Vec3,
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
    rotation: F32Vec3,
    coord: U8Vec3,
    position: F32Vec3,
    waypoint_property: Option<&'a WaypointSpecialProperty>,
    pivot_position: F32Vec3,
    scale: f32,
}

struct Chunk4;

struct Chunk5;

impl AnchoredObject {
    pub fn item_model_id(&self) -> &str {
        self.0.borrow_chunks().chunk_2.item_model_id
    }

    pub fn rotation(&self) -> &F32Vec3 {
        &self.0.borrow_chunks().chunk_2.rotation
    }

    pub fn coord(&self) -> &U8Vec3 {
        &self.0.borrow_chunks().chunk_2.coord
    }

    pub fn position(&self) -> &F32Vec3 {
        &self.0.borrow_chunks().chunk_2.position
    }

    pub fn waypoint_property(&self) -> Option<&WaypointSpecialProperty> {
        self.0.borrow_chunks().chunk_2.waypoint_property
    }

    pub fn pivot_position(&self) -> &F32Vec3 {
        &self.0.borrow_chunks().chunk_2.pivot_position
    }

    pub fn scale(&self) -> f32 {
        self.0.borrow_chunks().chunk_2.scale
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
        let rotation = r.vec3_f32()?;
        let coord = r.vec3_u8()?;
        let _anchor_tree_id = r.id_or_null()?;
        let position = r.vec3_f32()?;
        let waypoint_property = r.node_ref_or_null()?;
        let flags = r.u16()?;
        let pivot_position = r.vec3_f32()?;
        let scale = r.f32()?;

        if flags & 0x0004 != 0 {
            todo!()
        }

        r.vec3_f32()?;
        r.vec3_f32()?;

        Ok(Self {
            item_model_id,
            rotation,
            coord,
            position,
            waypoint_property,
            pivot_position,
            scale,
        })
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
