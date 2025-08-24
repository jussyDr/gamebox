use crate::read::{BodyReader, ReadEnum, ReadNode, Result, read_body_chunks};

pub struct EffectSimi {
    chunk_5: Chunk5,
}

struct Chunk5;

enum ColorBlendMode {
    Set,
    Mult,
}

impl ReadEnum for ColorBlendMode {
    fn from_u32(index: u32) -> Result<Self> {
        match index {
            0 => Ok(Self::Set),
            1 => Ok(Self::Mult),
            _ => todo!("{index}"),
        }
    }
}

impl ReadNode for EffectSimi {
    const CLASS_ID: u32 = 0x07010000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_5: r.chunk(0x07010005, |r| {
                    let _keys = r.list(|r| {
                        let _time = r.f32()?;
                        let _position = r.vec2_f32()?;
                        let _rotation = r.f32()?;
                        let _scale = r.vec2_f32()?;
                        let _opacity = r.f32()?;
                        let _depth = r.f32()?;
                        r.f32()?;
                        r.f32()?;
                        r.f32()?;
                        r.f32()?;

                        Ok(())
                    })?;
                    let _centered = r.bool32()?;
                    let _color_blend_mode = r.enum32::<ColorBlendMode>()?;
                    let _is_continuous_effect = r.bool32()?;
                    let _is_interpolated = r.bool32()?;

                    Ok(Chunk5)
                })?,
            })
        })
    }
}
