use crate::{
    game::ctn::FileRef,
    read::{BodyReader, ReadNode, Result, read_body_chunks},
};

pub struct ColorGrading {
    chunk_0: Chunk0,
    chunk_1: Chunk1,
}

struct Chunk0;

struct Chunk1 {
    keys: Box<[Key]>,
}

struct Key;

impl ReadNode for ColorGrading {
    const CLASS_ID: u32 = 0x03186000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_0: r.chunk(0x03186000, |r| {
                    let _image = FileRef::read(r)?;

                    Ok(Chunk0)
                })?,
                chunk_1: r.chunk(0x03186001, |r| {
                    let keys = r.list(|r| {
                        let _time = r.f32()?;
                        let _intensity = r.f32()?;

                        Ok(Key)
                    })?;

                    Ok(Chunk1 { keys })
                })?,
            })
        })
    }
}
