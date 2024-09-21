use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunksInline},
    Error, Reader,
};

/// A traits metadata.
#[derive(Default)]
pub struct TraitsMetadata;

impl BodyChunksInline for TraitsMetadata {
    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(0, |n, r| Self::read_chunk_0(n, r), false)];

        chunks.into_iter()
    }
}

impl TraitsMetadata {
    fn read_chunk_0<N, I>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 6 {
            return Err(Error);
        }

        let type_count = read_varuint(r)?;
        let types = r.repeat(type_count as usize, |r| Type::read(r))?;
        let trait_count = read_varuint(r)?;

        for _ in 0..trait_count {
            let len = read_varuint(r)?;
            let _trait_name = r.string_of_len(len as usize)?;
            let type_index = read_varuint(r)?;
            let ty = types.get(type_index as usize).ok_or(Error)?;
            read_value(r, ty)?;
        }

        Ok(())
    }
}

fn read_varuint<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<u32, Error> {
    let first = r.u8()? as u32;

    if first > 127 {
        let second = r.u16()? as u32;

        Ok(first & 127 | second << 7)
    } else {
        Ok(first)
    }
}

enum Type {
    Void,
    Boolean,
    Integer,
    Real,
    Class,
    Text,
    Enum,
    Array(Box<ArrayType>),
    ParamArray,
    Vec2,
    Vec3,
    Int3,
    Iso4,
    Ident,
    Int2,
    Struct(Box<StructType>),
    ValueNotComputed,
}

struct ArrayType {
    key_type: Type,
    value_type: Type,
}

struct StructType {
    members: Box<[StructMemberType]>,
}

struct StructMemberType {
    ty: Type,
}

impl Type {
    fn read<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Self, Error> {
        let ty = match r.u8()? {
            0 => Self::Void,
            1 => Self::Boolean,
            2 => Self::Integer,
            3 => Self::Real,
            4 => Self::Class,
            5 => Self::Text,
            6 => Self::Enum,
            7 => {
                let key_type = Self::read(r)?;
                let value_type = Self::read(r)?;

                Self::Array(Box::new(ArrayType {
                    key_type,
                    value_type,
                }))
            }
            8 => Self::ParamArray,
            9 => Self::Vec2,
            10 => Self::Vec3,
            11 => Self::Int3,
            12 => Self::Iso4,
            13 => Self::Ident,
            14 => Self::Int2,

            15 => {
                let member_count = r.u8()?;
                let _name = r.string()?;
                let members = r.repeat(member_count as usize, |r| {
                    let _name = r.string()?;
                    let ty = Self::read(r)?;

                    Ok(StructMemberType { ty })
                })?;

                Self::Struct(Box::new(StructType { members }))
            }
            16 => Self::ValueNotComputed,
            _ => return Err(Error),
        };

        Ok(ty)
    }
}

fn read_value<I, N>(r: &mut Reader<impl Read, I, N>, ty: &Type) -> Result<(), Error> {
    match *ty {
        Type::Boolean => {
            r.bool8()?;
        }
        Type::Integer => {
            r.i32()?;
        }
        Type::Real => {
            r.f32()?;
        }
        Type::Text => {
            let len = read_varuint(r)?;
            r.string_of_len(len as usize)?;
        }
        Type::Array(ref array_type) => {
            let array_field_count = read_varuint(r)?;

            if matches!(array_type.key_type, Type::Void) {
                let _array = r.repeat(array_field_count as usize, |r| {
                    read_value(r, &array_type.value_type)
                })?;
            } else {
                let _dictionary = r.repeat(array_field_count as usize, |r| {
                    read_value(r, &array_type.key_type)?;
                    read_value(r, &array_type.value_type)?;

                    Ok(())
                })?;
            }
        }
        Type::Vec2 => {
            r.vec2::<f32>()?;
        }
        Type::Vec3 => {
            r.vec3::<f32>()?;
        }
        Type::Int2 => {
            r.vec2::<i32>()?;
        }
        Type::Int3 => {
            r.vec3::<i32>()?;
        }
        Type::Struct(ref struct_type) => {
            for member_type in &struct_type.members {
                read_value(r, &member_type.ty)?;
            }
        }
        _ => return Err(Error),
    }

    Ok(())
}
