use std::sync::Arc;

use crate::{
    game::{WaypointSpecialProperty, ctn::FileRef},
    read::{BodyReader, Error, ReadNode, Result, read_body_chunks},
};

pub struct AnchoredObject {
    chunk_2: Chunk2,
    chunk_4: Chunk4,
    chunk_5: Chunk5,
}

struct Chunk2;

struct Chunk4;

struct Chunk5;

impl ReadNode for AnchoredObject {
    const CLASS_ID: u32 = 0x03101000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_2: r.chunk(0x03101002, |r| {
                    if r.u32()? != 8 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _item_model_id = r.string_ref()?;
                    let _item_collection = r.string_ref()?;
                    let _item_author = r.string_ref()?;
                    let _yaw_pitch_roll = r.yaw_pitch_roll()?;
                    let _block_unit_coord = r.vec3_u8()?;
                    let _anchor_tree_id = r.string_ref()?;
                    let _absolute_position_in_map = r.vec3_f32()?;
                    let _waypoint_special_property =
                        r.node_ref::<Arc<WaypointSpecialProperty>>()?;
                    let flags = r.u16()?;
                    let _pivot_position = r.vec3_f32()?;
                    let _scale = r.f32()?;

                    if flags & 0x0004 != 0 {
                        let _file_ref = FileRef::read(r)?;
                    }

                    r.vec3_f32()?;
                    r.vec3_f32()?;

                    Ok(Chunk2)
                })?,
                chunk_4: r.chunk_skippable(0x03101004, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    r.u32()?;

                    Ok(Chunk4)
                })?,
                chunk_5: r.chunk_skippable(0x03101005, |r| {
                    r.u32()?;
                    r.u32()?;
                    r.u8()?;

                    Ok(Chunk5)
                })?,
            })
        })
    }
}
