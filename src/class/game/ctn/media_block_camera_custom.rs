use std::{any::Any, cell::OnceCell, marker::PhantomData, sync::Arc};

use ouroboros::self_referencing;

use crate::read::{BodyChunksReader, BodyReader, ClassId, Error};

pub struct MediaBlockCameraCustom(Inner);

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
    chunk_6: Chunk6,
}

struct Chunk6;

impl ClassId for MediaBlockCameraCustom {
    const CLASS_ID: u32 = 0x030a2000;
}

impl MediaBlockCameraCustom {
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

                let chunk_6 = r.chunk(0x030a2006, |r| {
                    let version = r.u32()?;

                    if version != 3 {
                        return Err(Error::new(format!("unknown chunk version: {version}")));
                    }

                    let _keys = r.list(|r| {
                        let _time = r.f32()?;
                        let _interpolation = r.u32()?;
                        let _anchor_rot = r.bool32()?;
                        let _anchor = r.u32()?;
                        let _anchor_vis = r.bool32()?;
                        let _target = r.u32()?;
                        let _position = r.vec3_f32()?;
                        let _pitch_yaw_roll = r.vec3_f32()?;
                        let _fov = r.f32()?;
                        let _target_position = r.vec3_f32()?;
                        let _near_z = r.f32()?;

                        let _position = r.vec3_f32()?;
                        let _pitch_yaw_roll = r.vec3_f32()?;
                        let _fov = r.f32()?;
                        let _target_position = r.vec3_f32()?;
                        let _near_z = r.f32()?;

                        let _position = r.vec3_f32()?;
                        let _pitch_yaw_roll = r.vec3_f32()?;
                        let _fov = r.f32()?;
                        let _target_position = r.vec3_f32()?;
                        let _near_z = r.f32()?;

                        Ok(())
                    })?;

                    Ok(Chunk6)
                })?;

                r.end()?;

                Ok(Chunks {
                    delme: PhantomData,
                    chunk_6,
                })
            },
        };

        builder.try_build().map(Self)
    }
}
