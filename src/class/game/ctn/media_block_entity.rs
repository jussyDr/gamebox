use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    class::plug::EntRecordData,
    game::ctn::FileRef,
    read::{BodyChunksReader, BodyReader, ClassId, Error},
};

pub struct MediaBlockEntity(Inner);

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
}

struct Chunk0 {
    keys: Box<[Key]>,
}

pub struct Key;

impl MediaBlockEntity {
    pub fn keys(&self) -> &[Key] {
        &self.0.borrow_chunks().chunk_0.keys
    }
}

impl ClassId for MediaBlockEntity {
    const CLASS_ID: u32 = 0x0329f000;
}

impl MediaBlockEntity {
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

                let chunk_0 = r.chunk(0x0329f000, Chunk0::read)?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_0,
                })
            },
        };

        builder.try_build().map(Self)
    }
}

impl Chunk0 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let version = r.u32()?;

        if version != 6 {
            return Err(Error::new(format!("unknown chunk version: {version}")));
        }

        let _record_data = r.node_ref::<EntRecordData>()?;
        let _start_offset = r.f32()?;
        let _notice_records = r.list(|r| r.u32())?;
        let _no_damage = r.bool32()?;
        r.bool32()?;
        let _force_light = r.u32()?;
        let _force_hue = r.bool32()?;
        let _player_model_id = r.id_or_null()?;
        let _player_model_collection = r.id_or_null()?;
        let _player_model_author = r.id_or_null()?;
        r.vec3_f32()?;
        let _skin_names = r.list(|r| FileRef::read(r))?;

        if r.bool32()? {
            todo!()
        }

        let keys = r.list(|r| {
            let _time = r.f32()?;
            let _lights = r.u32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            let _trail_intensity = r.f32()?;

            Ok(Key)
        })?;

        Ok(Self { keys })
    }
}
