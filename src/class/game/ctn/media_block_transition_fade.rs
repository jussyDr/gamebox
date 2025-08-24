use crate::read::{BodyReader, ReadNode, Result, read_body_chunks};

pub struct MediaBlockTransitionFade {
    chunk_0: Chunk0,
}

struct Chunk0;

impl ReadNode for MediaBlockTransitionFade {
    const CLASS_ID: u32 = 0x030ab000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_0: r.chunk(0x030ab000, |r| {
                    let _keys = r.list(|r| {
                        let _time = r.f32()?;
                        let _opacity = r.f32()?;

                        Ok(())
                    })?;
                    let _color = r.vec3_f32()?;
                    r.f32()?;

                    Ok(Chunk0)
                })?,
            })
        })
    }
}
