use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::read::{BodyChunksReader, BodyReader, ClassId, Error, ReadNode};

pub struct EffectSimi(Inner);

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
    chunk_5: Chunk5,
}

struct Chunk5;

impl ClassId for EffectSimi {
    const CLASS_ID: u32 = 0x07010000;
}

impl ReadNode for EffectSimi {
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

                let chunk_5 = r.chunk(0x07010005, |r| {
                    let _keys = r.list(|r| {
                        let _time = r.f32()?;
                        let _position = r.vec2_f32()?;
                        let _rotation = r.f32()?;
                        let _scale = r.vec2_f32()?;
                        let _opacity = r.f32()?;
                        let _depth = r.f32()?;
                        r.f32()?;
                        r.f32()?;
                        r.f32()?;
                        r.f32()?;

                        Ok(())
                    })?;
                    let _centered = r.bool32()?;
                    let _color_blend_mode = r.u32()?;
                    let _is_continuous_effect = r.bool32()?;
                    let _is_interpolated = r.bool32()?;

                    Ok(Chunk5)
                })?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_5,
                })
            },
        };

        builder.try_build().map(Self)
    }
}
