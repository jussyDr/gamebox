use crate::{
    Vec3,
    read::{BodyReader, Error, ReadNode, Reader, Result, read_body_chunks},
};

pub struct TraitsMetadata {
    chunk_0: Chunk0,
}

struct Chunk0 {
    traits: Box<[Trait]>,
}

impl TraitsMetadata {
    /// Traits.
    pub fn traits(&self) -> &[Trait] {
        &self.chunk_0.traits
    }
}

pub struct Trait {
    name: String,
    value: Value,
}

impl ReadNode for TraitsMetadata {
    const CLASS_ID: u32 = 0x11002000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        read_body_chunks(r, |r| {
            let chunk_0 = r.chunk(0x11002000, |r| {
                if r.u32()? != 6 {
                    return Err(Error::Internal("unknown chunk version".into()));
                }

                let num_types = read_u32_packed(r)?;
                let types = r.repeat(num_types as usize, |r| read_type(r))?;
                let trait_count = read_u32_packed(r)?;
                let traits = r.repeat(trait_count as usize, |r| {
                    let name_len = read_u32_packed(r)?;
                    let name = r.string_of_len(name_len as usize)?;
                    let type_index = read_u32_packed(r)?;
                    let ty = types
                        .get(type_index as usize)
                        .ok_or_else(|| Error::Internal("type index out of bounds".into()))?;
                    let value = read_value(r, ty)?;

                    Ok(Trait { name, value })
                })?;

                Ok(Chunk0 { traits })
            })?;

            Ok(Self { chunk_0 })
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

#[derive(Debug)]
enum Type {
    Array {
        key_type: Box<Type>,
        value_type: Box<Type>,
    },
    Bool,
    Integer,
    Real,
    Text,
    Vec3,
    Void,
}

fn read_type(r: &mut impl Reader) -> Result<Type> {
    match r.u8()? {
        0 => Ok(Type::Void),
        1 => Ok(Type::Bool),
        2 => Ok(Type::Integer),
        3 => Ok(Type::Real),
        5 => Ok(Type::Text),
        7 => {
            let key_type = Box::new(read_type(r)?);
            let value_type = Box::new(read_type(r)?);

            Ok(Type::Array {
                key_type,
                value_type,
            })
        }
        10 => Ok(Type::Vec3),
        index => todo!("{index}"),
    }
}

enum Value {
    Array(Vec<(Value, Value)>),
    Bool(bool),
    Integer(i32),
    Real(f32),
    Text(String),
    Vec3(Vec3<f32>),
    Void,
}

fn read_value(r: &mut impl Reader, ty: &Type) -> Result<Value> {
    match ty {
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
        Type::Bool => {
            let value = r.bool8()?;

            Ok(Value::Bool(value))
        }
        Type::Integer => {
            let value = r.i32()?;

            Ok(Value::Integer(value))
        }
        Type::Real => {
            let value = r.f32()?;

            Ok(Value::Real(value))
        }
        Type::Text => {
            let len = read_u32_packed(r)?;
            let text = r.string_of_len(len as usize)?;

            Ok(Value::Text(text))
        }
        Type::Vec3 => {
            let value = r.vec3_f32()?;

            Ok(Value::Vec3(value))
        }
        Type::Void => Ok(Value::Void),
    }
}
