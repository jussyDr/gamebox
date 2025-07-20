use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::read::{BodyChunksReader, BodyReader, ClassId, Error};

pub struct MediaBlockCameraGame(Inner);

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
    chunk_7: Chunk7,
}

struct Chunk7;

impl ClassId for MediaBlockCameraGame {
    const CLASS_ID: u32 = 0x03084000;
}

impl MediaBlockCameraGame {
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

                let chunk_7 = r.chunk(0x03084007, Chunk7::read)?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_7,
                })
            },
        };

        builder.try_build().map(Self)
    }
}

impl Chunk7 {
    fn read(r: &mut BodyReader) -> Result<Self, Error> {
        let version = r.u32()?;

        if version != 4 {
            return Err(Error::new(format!("unknown chunk version: {version}")));
        }

        let _start = r.f32()?;
        let _end = r.f32()?;
        let _game_cam = r.u32()?;
        let _clip_ent_id = r.u32()?;
        let _cam_position = r.vec3_f32()?;
        let _cam_pitch_yaw_roll = r.vec3_f32()?;
        let _cam_fov = r.f32()?;
        r.f32()?;
        r.f32()?;
        let _cam_near_clip_plane = r.f32()?;
        let _cam_far_clip_plane = r.f32()?;
        r.bool32()?;
        r.bool32()?;
        r.bool32()?;
        r.f32()?;
        r.u32()?;

        Ok(Self)
    }
}
