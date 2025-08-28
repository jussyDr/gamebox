use crate::{
    game::ctn::media::Block,
    read::{BodyReader, Error, ReadNode, Result, read_body_chunks},
};

pub struct Track {
    chunk_1: Chunk1,
    chunk_5: Chunk5,
}

struct Chunk1;

struct Chunk5;

impl ReadNode for Track {
    const CLASS_ID: u32 = 0x03078000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_1: r.chunk(0x03078001, |r| {
                    let _name = r.string()?;
                    let _blocks = r.list_versioned(|r| r.node_ref::<Block>())?;
                    r.u32()?;

                    Ok(Chunk1)
                })?,
                chunk_5: r.chunk(0x03078005, |r| {
                    if r.u32()? != 1 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _is_keep_playing = r.bool32()?;
                    let _is_read_only = r.bool32()?;
                    let _is_cycling = r.bool32()?;
                    let _repeating_segment_start = r.f32()?;
                    let _repeating_segment_end = r.f32()?;

                    Ok(Chunk5)
                })?,
            })
        })
    }
}
