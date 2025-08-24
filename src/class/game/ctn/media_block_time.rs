use crate::read::{BodyReader, ReadNode, Result, read_body_chunks};

pub struct MediaBlockTime {
    chunk_0: Chunk0,
}

struct Chunk0;

impl ReadNode for MediaBlockTime {
    const CLASS_ID: u32 = 0x03085000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_0: r.chunk(0x03085000, |r| {
                    let _keys = r.list(|r| {
                        let _time = r.f32()?;
                        let _time_value = r.f32()?;
                        let _tangent = r.f32()?;

                        Ok(())
                    })?;

                    Ok(Chunk0)
                })?,
            })
        })
    }
}
