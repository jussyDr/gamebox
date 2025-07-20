use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::read::{BodyChunksReader, BodyReader, ClassId, Error, ReadNode};

pub struct WaypointSpecialProperty(Inner);

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
    chunk_0: Chunk0,
    chunk_1: Chunk1,
}

struct Chunk0;

struct Chunk1;

pub enum Property {
    Checkpoint,
    Goal,
    Spawn,
}

impl ClassId for WaypointSpecialProperty {
    const CLASS_ID: u32 = 0x2e009000;
}

impl ReadNode for WaypointSpecialProperty {
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

                let chunk_0 = r.chunk(0x2e009000, Chunk0::read)?;
                let chunk_1 = r.skippable_chunk(0x2e009001, Chunk1::read)?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_0,
                    chunk_1,
                })
            },
        };

        builder.try_build().map(Self)
    }
}

impl Chunk0 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let version = r.u32()?;

        if version != 2 {
            return Err(Error::new(format!("unknown chunk version: {version}")));
        }

        let _property = match r.string()? {
            "Checkpoint" => Property::Checkpoint,
            "Goal" => Property::Goal,
            "Spawn" => Property::Spawn,
            tag => todo!("{tag}"),
        };

        let _value = r.u32()?;

        Ok(Self)
    }
}

impl Chunk1 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let version = r.u32()?;

        if version != 0 {
            return Err(Error::new(format!("unknown chunk version: {version}")));
        }

        if r.bool32()? {
            todo!();
        }

        Ok(Self)
    }
}
