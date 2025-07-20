use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::read::{BodyChunksReader, BodyReader, ClassId, Error};

pub struct MediaBlockFog(Inner);

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

impl MediaBlockFog {
    pub fn keys(&self) -> &[Key] {
        &self.0.borrow_chunks().chunk_0.keys
    }
}

impl ClassId for MediaBlockFog {
    const CLASS_ID: u32 = 0x03199000;
}

impl MediaBlockFog {
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

                let chunk_0 = r.chunk(0x03199000, Chunk0::read)?;

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

        if version != 2 {
            return Err(Error::new(format!("unknown chunk version: {version}")));
        }

        let keys = r.list(|r| {
            let _time = r.f32()?;
            let _intensity = r.f32()?;
            let _sky_intensity = r.f32()?;
            let _distance = r.f32()?;
            let _coefficient = r.f32()?;
            let _color = r.vec3_f32()?;
            let _clouds_opacity = r.f32()?;
            let _clouds_speed = r.f32()?;

            Ok(Key)
        })?;

        Ok(Self { keys })
    }
}
