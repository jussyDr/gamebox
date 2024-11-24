use crate::Class;

#[derive(Default)]
pub struct TraitsMetadata;

impl Class for TraitsMetadata {
    const CLASS_ID: u32 = 0x11002000;
}

pub enum Type {
    Void,
    Boolean,
    Integer,
    Real,
    Class,
    Text,
    Enum,
    Array {
        key_type: Box<Type>,
        value_type: Box<Type>,
    },
    ParamArray,
    Vec2,
    Vec3,
    Int3,
    Iso4,
    Ident,
    Int2,
    Struct {
        member_types: Vec<Type>,
    },
    ValueNotComputed,
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{TraitsMetadata, Type};

    impl ReadBody for TraitsMetadata {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut crate::read::reader::Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for TraitsMetadata {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl TraitsMetadata {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(Error::version("traits metadata", version));
            }

            let type_count = read_len(r)? as usize;
            let types = r.repeat(type_count, |r| read_type(r))?;
            let trait_count = read_len(r)? as usize;
            let _traits = r.repeat(trait_count, |r| {
                let name_len = read_len(r)? as usize;
                let _name = r.bytes(name_len);
                let type_index = read_len(r)? as usize;
                read_value(r, &types[type_index])?;

                Ok(())
            })?;

            Ok(())
        }
    }

    fn read_len<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<u32, Error> {
        let first_byte = r.u8()?;

        let second_short = if first_byte > 127 { r.u16()? } else { 0 };

        Ok((first_byte & 127) as u32 | (second_short as u32) << 7)
    }

    fn read_type<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Type, Error> {
        match r.u8()? {
            0 => Ok(Type::Void),
            1 => Ok(Type::Boolean),
            2 => Ok(Type::Integer),
            3 => Ok(Type::Real),
            4 => Ok(Type::Class),
            5 => Ok(Type::Text),
            6 => Ok(Type::Enum),
            7 => {
                let key_type = read_type(r)?;
                let value_type = read_type(r)?;

                Ok(Type::Array {
                    key_type: Box::new(key_type),
                    value_type: Box::new(value_type),
                })
            }
            8 => Ok(Type::ParamArray),
            9 => Ok(Type::Vec2),
            10 => Ok(Type::Vec3),
            11 => Ok(Type::Int3),
            12 => Ok(Type::Iso4),
            13 => Ok(Type::Ident),
            14 => Ok(Type::Int2),
            15 => {
                let member_count = r.u8()?;
                let _name = r.string()?;

                let mut member_types = vec![];

                for _ in 0..member_count {
                    let _name = r.string()?;
                    let ty = read_type(r)?;

                    member_types.push(ty);
                }

                Ok(Type::Struct { member_types })
            }
            16 => Ok(Type::ValueNotComputed),
            _ => todo!(),
        }
    }

    fn read_value<I, N>(r: &mut Reader<impl Read, I, N>, ty: &Type) -> Result<(), Error> {
        match ty {
            Type::Boolean => {
                r.bool()?;
            }
            Type::Integer => {
                r.i32()?;
            }
            Type::Real => {
                r.f32()?;
            }
            Type::Text => {
                let len = read_len(r)?;
                r.bytes(len as usize)?;
            }
            Type::Array {
                key_type,
                value_type,
            } => {
                let len = read_len(r)?;

                for _ in 0..len {
                    read_value(r, key_type)?;
                    read_value(r, value_type)?;
                }
            }
            Type::Vec2 => {
                r.f32()?;
                r.f32()?;
            }
            Type::Vec3 => {
                r.f32()?;
                r.f32()?;
                r.f32()?;
            }
            Type::Int2 => {
                r.i32()?;
                r.i32()?;
            }
            Type::Int3 => {
                r.i32()?;
                r.i32()?;
                r.i32()?;
            }
            Type::Struct { member_types } => {
                for member_type in member_types {
                    read_value(r, member_type)?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}
