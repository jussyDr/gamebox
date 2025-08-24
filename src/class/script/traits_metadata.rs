use crate::read::{BodyReader, Error, ReadNode, Reader, Result, read_body_chunks};

pub struct TraitsMetadata {
    chunk_0: Chunk0,
}

struct Chunk0;

impl ReadNode for TraitsMetadata {
    const CLASS_ID: u32 = 0x11002000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            Ok(Self {
                chunk_0: r.chunk(0x11002000, |r| {
                    if r.u32()? != 6 {
                        return Err(Error::Internal("unknown chunk version".into()));
                    }

                    let num_types = read_u32_packed(r)?;
                    let types = r.repeat(num_types as usize, |r| read_type(r))?;
                    let trait_count = read_u32_packed(r)?;
                    let _traits = r.repeat(trait_count as usize, |r| {
                        let name_len = read_u32_packed(r)?;
                        let _name = r.repeat_u8(name_len as usize)?;
                        let type_index = read_u32_packed(r)?;
                        let ty = types
                            .get(type_index as usize)
                            .ok_or_else(|| Error::Internal("".into()))?;
                        let _value = read_value(r, ty)?;

                        Ok(())
                    })?;

                    Ok(Chunk0)
                })?,
            })
        })
    }
}

fn read_u32_packed(r: &mut impl Reader) -> Result<u32> {
    let mut value = r.u8()? as u32;

    if value > 127 {
        value |= (r.u16()? as u32) << 7;
    }

    Ok(value)
}

enum Type {
    Array {
        key_type: Box<Type>,
        value_type: Box<Type>,
    },
    Integer,
    Void,
}

fn read_type(r: &mut impl Reader) -> Result<Type> {
    match r.u8()? {
        0 => Ok(Type::Void),
        2 => Ok(Type::Integer),
        7 => {
            let key_type = Box::new(read_type(r)?);
            let value_type = Box::new(read_type(r)?);

            Ok(Type::Array {
                key_type,
                value_type,
            })
        }
        index => todo!("{index}"),
    }
}

enum Value {
    Void,
    Integer(i32),
    Array(Vec<(Value, Value)>),
}

fn read_value(r: &mut impl Reader, ty: &Type) -> Result<Value> {
    match ty {
        Type::Void => Ok(Value::Void),
        Type::Integer => {
            let value = r.i32()?;

            Ok(Value::Integer(value))
        }
        Type::Array {
            key_type,
            value_type,
        } => {
            let len = read_u32_packed(r)?;

            let mut array = vec![];

            for _ in 0..len {
                let key = read_value(r, key_type)?;
                let value = read_value(r, value_type)?;

                array.push((key, value));
            }

            Ok(Value::Array(array))
        }
    }
}
