use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::{
    game::ctn::MediaClip,
    read::{BodyChunksReader, BodyReader, ClassId, Error, ReadNode},
};

pub struct MediaClipGroup(Inner);

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
    chunk_3: Chunk3,
}

struct Chunk3;

impl ClassId for MediaClipGroup {
    const CLASS_ID: u32 = 0x0307a000;
}

impl ReadNode for MediaClipGroup {
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

                let chunk_3 = r.chunk(0x0307a003, |r| {
                    let _clips = r.list_with_version(|r| r.node_ref::<MediaClip>())?;
                    let _triggers = r.list(|r| {
                        r.u32()?;
                        r.u32()?;
                        r.u32()?;
                        r.u32()?;
                        let _condition = r.u32()?;
                        let _condition_value = r.f32()?;
                        let _coords = r.list(|r| r.vec3_u32())?;

                        Ok(())
                    })?;

                    Ok(Chunk3)
                })?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_3,
                })
            },
        };

        builder.try_build().map(Self)
    }
}
