use std::marker::PhantomData;

use crate::read::{BodyChunksReader, BodyReader, Error};

pub struct TraitsMetadata<'a> {
    delme: PhantomData<&'a ()>,
    chunk_0: Chunk0,
}

struct Chunks<'a> {
    delme: PhantomData<&'a ()>,
    chunk_0: Chunk0,
}

struct Chunk0;

enum ScriptType {
    Void,
    Integer,
    Array {
        key: Box<ScriptType>,
        value: Box<ScriptType>,
    },
}

impl<'a> TraitsMetadata<'a> {
    pub fn read(r: &mut BodyReader<'a, '_>) -> Result<Self, Error> {
        let mut r = BodyChunksReader(r);

        let chunk_0 = r.chunk(0x11002000, |r| {
            let version = r.u32()?;

            if version != 6 {
                return Err(Error::new(format!("unknown chunk version: {version}")));
            }

            let num_types = read_packed_u32(r)?;
            let types = r.repeat(num_types as usize, |r| read_type(r))?;
            let num_traits = read_packed_u32(r)?;
            r.repeat(num_traits as usize, |r| {
                let name_len = read_packed_u32(r)?;
                let _name = r.repeat_u8(name_len as usize)?;
                let type_index = read_packed_u32(r)?;
                read_value(r, &types[type_index as usize])?;

                Ok(())
            })?;

            Ok(Chunk0)
        })?;

        r.end()?;

        Ok(Self {
            delme: PhantomData,
            chunk_0,
        })
    }
}

fn read_packed_u32(r: &mut BodyReader) -> Result<u32, Error> {
    let x = r.u8()?;
    let y = if x >= 0x80 { r.u16()? } else { 0 };

    Ok((x & 0x7f) as u32 | (y as u32) << 7)
}

fn read_type(r: &mut BodyReader) -> Result<ScriptType, Error> {
    match r.u8()? {
        0 => Ok(ScriptType::Void),
        2 => Ok(ScriptType::Integer),
        7 => {
            let key = Box::new(read_type(r)?);
            let value = Box::new(read_type(r)?);

            Ok(ScriptType::Array { key, value })
        }
        value => Err(Error::new(format!(
            "unknown variant of enum script type: {value}"
        ))),
    }
}

fn read_value(r: &mut BodyReader, ty: &ScriptType) -> Result<(), Error> {
    match ty {
        ScriptType::Void => {}
        ScriptType::Integer => {
            r.u32()?;
        }
        ScriptType::Array { key, value } => {
            let len = read_packed_u32(r)?;
            r.repeat(len as usize, |r| {
                read_value(r, key)?;
                read_value(r, value)?;

                Ok(())
            })?;
        }
    }

    Ok(())
}
