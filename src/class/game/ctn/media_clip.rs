use std::sync::Arc;

use crate::{
    game::ctn::MediaTrack,
    read::{BodyReader, Error, ReadNode, Result, read_body_chunks},
};

pub struct MediaClip {
    chunk_13: Chunk13,
}

struct Chunk13;

impl ReadNode for MediaClip {
    const CLASS_ID: u32 = 0x03079000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_13: r.chunk(0x0307900d, |r| {
                    if r.u32()? != 0 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _tracks = r.list_versioned(|r| r.node_ref::<Arc<MediaTrack>>())?;
                    let _name = r.string()?;
                    let _stop_when_leave = r.bool32()?;
                    r.bool32()?;
                    let _stop_when_respawn = r.bool32()?;
                    r.string()?;
                    r.f32()?;
                    let _local_player_clip_ent_index = r.u32()?;

                    Ok(Chunk13)
                })?,
            })
        })
    }
}
