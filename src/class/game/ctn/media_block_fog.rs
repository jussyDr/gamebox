use crate::read::{BodyReader, Error, ReadNode, Result, read_body_chunks};

pub struct MediaBlockFog {
    chunk_0: Chunk0,
}

struct Chunk0;

impl ReadNode for MediaBlockFog {
    const CLASS_ID: u32 = 0x03199000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_0: r.chunk(0x03199000, |r| {
                    if r.u32()? != 2 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let _keys = r.list(|r| {
                        let _time = r.f32()?;
                        let _intensity = r.f32()?;
                        let _sky_intensity = r.f32()?;
                        let _distance = r.f32()?;
                        let _coefficient = r.f32()?;
                        let _color = r.rgb_f32()?;
                        let _clouds_opacity = r.f32()?;
                        let _clouds_speed = r.f32()?;

                        Ok(())
                    })?;

                    Ok(Chunk0)
                })?,
            })
        })
    }
}
