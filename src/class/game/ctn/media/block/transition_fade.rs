use crate::{
    Rgb,
    read::{BodyReader, ReadNode, Result, read_body_chunks},
};

pub struct TransitionFade {
    chunk_0: Chunk0,
}

struct Chunk0 {
    keys: Box<[Key]>,
    color: Rgb,
}

struct Key;

impl TransitionFade {
    pub fn keys(&self) -> &[Key] {
        &self.chunk_0.keys
    }

    /// Fade color.
    pub fn color(&self) -> Rgb {
        self.chunk_0.color
    }
}

impl ReadNode for TransitionFade {
    const CLASS_ID: u32 = 0x030ab000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            let chunk_0 = r.chunk(0x030ab000, |r| {
                let keys = r.list(|r| {
                    let _time = r.f32()?;
                    let _opacity = r.f32()?;

                    Ok(Key)
                })?;
                let color = r.rgb()?;
                r.f32()?;

                Ok(Chunk0 { keys, color })
            })?;

            Ok(Self { chunk_0 })
        })
    }
}
