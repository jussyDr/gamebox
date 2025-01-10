//! Traits metadata.

use std::collections::HashMap;

use crate::{Class, Int2, Int3, Iso4, Vec2, Vec3};

/// Traits metadata.
#[derive(Default)]
pub struct TraitsMetadata {
    traits: HashMap<String, Trait>,
}

impl Class for TraitsMetadata {
    const CLASS_ID: u32 = 0x11002000;
}

impl TraitsMetadata {
    /// Metadata traits.
    pub const fn traits(&self) -> &HashMap<String, Trait> {
        &self.traits
    }
}

/// A trait.
pub enum Trait {
    /// Void.
    Void,
    /// Boolean.
    Boolean(bool),
    /// Integer.
    Integer(i32),
    /// Real number.
    Real(f32),
    /// Class.
    Class,
    /// Text.
    Text(String),
    /// Enum.
    Enum,
    /// Associative array.
    Array {
        /// Keys.
        keys: Vec<Trait>,
        /// Values.
        values: Vec<Trait>,
    },
    /// Parameter array.
    ParamArray,
    /// 2-dimensional vector.
    Vec2(Vec2),
    /// 3-dimensional vector.
    Vec3(Vec3),
    /// 3-dimensional vector.
    Int3(Int3),
    /// Iso4.
    Iso4(Iso4),
    /// Identifier.
    Ident,
    /// 2-dimensional vector.
    Int2(Int2),
    /// A struct.
    Struct {
        /// Name.
        name: String,
        /// Members.
        members: HashMap<String, Trait>,
    },
    /// Value not computed.
    ValueNotComputed,
}

#[derive(Debug)]
enum Type {
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
        name: String,
        member_types: Vec<(String, Type)>,
    },
    ValueNotComputed,
}

mod read {
    use std::{
        collections::HashMap,
        io::{Read, Seek},
    };

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::{Trait, TraitsMetadata, Type};

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
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl TraitsMetadata {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(Error::chunk_version(version));
            }

            let type_count = read_len(r)? as usize;
            let types = r.repeat(type_count, |r| read_type(r))?;
            let trait_count = read_len(r)? as usize;
            self.traits = HashMap::new();

            for _ in 0..trait_count {
                let name_len = read_len(r)? as usize;
                let name = r.string_of_len(name_len)?;
                let type_index = read_len(r)? as usize;
                let tr = read_contents(r, &types[type_index])?;

                self.traits.insert(name, tr);
            }

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
                let name = r.string()?;

                let mut member_types = vec![];

                for _ in 0..member_count {
                    let name = r.string()?;
                    let ty = read_type(r)?;

                    member_types.push((name, ty));
                }

                Ok(Type::Struct { name, member_types })
            }
            16 => Ok(Type::ValueNotComputed),
            value => Err(Error::enum_variant("script type", value as u32)),
        }
    }

    fn read_contents<I, N>(r: &mut Reader<impl Read, I, N>, ty: &Type) -> Result<Trait, Error> {
        match ty {
            Type::Void => Ok(Trait::Void),
            Type::Boolean => Ok(Trait::Boolean(r.bool8()?)),
            Type::Integer => Ok(Trait::Integer(r.i32()?)),
            Type::Real => Ok(Trait::Real(r.f32()?)),
            Type::Class => Ok(Trait::Class),
            Type::Text => {
                let len = read_len(r)? as usize;

                Ok(Trait::Text(r.string_of_len(len)?))
            }
            Type::Enum => Ok(Trait::Enum),
            Type::Array {
                key_type,
                value_type,
            } => {
                let len = read_len(r)?;

                let mut keys = vec![];
                let mut values = vec![];

                for _ in 0..len {
                    let key = read_contents(r, key_type)?;
                    let value = read_contents(r, value_type)?;

                    keys.push(key);
                    values.push(value);
                }

                Ok(Trait::Array { keys, values })
            }
            Type::ParamArray => Ok(Trait::ParamArray),
            Type::Vec2 => Ok(Trait::Vec2(r.vec2()?)),
            Type::Vec3 => Ok(Trait::Vec3(r.vec3()?)),
            Type::Int3 => Ok(Trait::Int3(r.int3()?)),
            Type::Iso4 => Ok(Trait::Iso4(r.iso4()?)),
            Type::Ident => Ok(Trait::Ident),
            Type::Int2 => Ok(Trait::Int2(r.int2()?)),
            Type::Struct { name, member_types } => {
                let mut members = HashMap::new();

                for (member_name, member_type) in member_types {
                    let member = read_contents(r, member_type)?;

                    members.insert(member_name.clone(), member);
                }

                Ok(Trait::Struct {
                    name: name.clone(),
                    members,
                })
            }
            Type::ValueNotComputed => Ok(Trait::ValueNotComputed),
        }
    }
}

mod write {
    use std::io::Write;

    use crate::write::{
        writable::{write_body_chunks, WriteBody},
        writer::{IdStateMut, NodeStateMut},
        BodyChunk, BodyChunks, Error, Writer,
    };

    use super::TraitsMetadata;

    impl WriteBody for TraitsMetadata {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for TraitsMetadata {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
