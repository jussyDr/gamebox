use crate::read::{BodyReader, ReadNode, Result, read_body_chunks};

pub struct DOF {
    chunk_2: Chunk2,
}

struct Chunk2 {
    keys: Box<[Key]>,
}

struct Key;

impl ReadNode for DOF {
    const CLASS_ID: u32 = 0x03126000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            let chunk_2 = r.chunk(0x03126002, |r| {
                let keys = r.list(|r| {
                    let _time = r.f32()?;
                    let _z_focus = r.f32()?;
                    let _lens_size = r.f32()?;
                    let _target = r.u32()?;
                    let _target_position = r.vec3_f32()?;

                    Ok(Key)
                })?;

                Ok(Chunk2 { keys })
            })?;

            Ok(DOF { chunk_2 })
        })
    }
}
