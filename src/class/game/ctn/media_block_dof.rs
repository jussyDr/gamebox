use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::read::{BodyChunksReader, BodyReader, ClassId, Error};

pub struct MediaBlockDOF(Inner);

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
}

struct Chunk2 {
    keys: Box<[Key]>,
}

pub struct Key;

impl MediaBlockDOF {
    pub fn keys(&self) -> &[Key] {
        &self.0.borrow_chunks().chunk_2.keys
    }
}

impl ClassId for MediaBlockDOF {
    const CLASS_ID: u32 = 0x03126000;
}

impl MediaBlockDOF {
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

                let chunk_2 = r.chunk(0x03126002, Chunk2::read)?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_2,
                })
            },
        };

        builder.try_build().map(Self)
    }
}

impl Chunk2 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let keys = r.list(|r| {
            let _time = r.f32()?;
            let _z_focus = r.f32()?;
            let _lens_size = r.f32()?;
            let _target = r.u32()?;
            let _target_position = r.vec3_f32()?;

            Ok(Key)
        })?;

        Ok(Self { keys })
    }
}
