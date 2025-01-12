//! Traits metadata.

use std::collections::HashMap;

use crate::{Class, Int2, Int3, Iso4, Vec2, Vec3};

/// Traits metadata.
#[derive(PartialEq, Default, Debug)]
pub struct TraitsMetadata {
    traits: HashMap<String, Value>,
}

impl Class for TraitsMetadata {
    const CLASS_ID: u32 = 0x11002000;
}

impl TraitsMetadata {
    /// Metadata traits.
    pub const fn traits(&self) -> &HashMap<String, Value> {
        &self.traits
    }
}

/// Value.
#[derive(PartialEq, Debug)]
pub enum Value {
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
    Array(ArrayValue),
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
        members: HashMap<String, Value>,
    },
    /// Value not computed.
    ValueNotComputed,
}

/// Array value.
#[derive(PartialEq, Debug)]
pub struct ArrayValue {
    key_type: Type,
    value_type: Type,
    keys: Vec<Value>,
    values: Vec<Value>,
}

impl ArrayValue {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Value> {
        &self.keys
    }

    /// Values.
    pub const fn values(&self) -> &Vec<Value> {
        &self.values
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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

    use super::{ArrayValue, TraitsMetadata, Type, Value};

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
                let tr = read_value(r, &types[type_index])?;

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

    fn read_value<I, N>(r: &mut Reader<impl Read, I, N>, ty: &Type) -> Result<Value, Error> {
        match ty {
            Type::Void => Ok(Value::Void),
            Type::Boolean => Ok(Value::Boolean(r.bool8()?)),
            Type::Integer => Ok(Value::Integer(r.i32()?)),
            Type::Real => Ok(Value::Real(r.f32()?)),
            Type::Class => Ok(Value::Class),
            Type::Text => {
                let len = read_len(r)? as usize;

                Ok(Value::Text(r.string_of_len(len)?))
            }
            Type::Enum => Ok(Value::Enum),
            Type::Array {
                key_type,
                value_type,
            } => {
                let len = read_len(r)?;

                let mut keys = vec![];
                let mut values = vec![];

                for _ in 0..len {
                    let key = read_value(r, key_type)?;
                    let value = read_value(r, value_type)?;

                    keys.push(key);
                    values.push(value);
                }

                Ok(Value::Array(ArrayValue {
                    key_type: *key_type.clone(),
                    keys,
                    value_type: *value_type.clone(),
                    values,
                }))
            }
            Type::ParamArray => Ok(Value::ParamArray),
            Type::Vec2 => Ok(Value::Vec2(r.vec2()?)),
            Type::Vec3 => Ok(Value::Vec3(r.vec3()?)),
            Type::Int3 => Ok(Value::Int3(r.int3()?)),
            Type::Iso4 => Ok(Value::Iso4(r.iso4()?)),
            Type::Ident => Ok(Value::Ident),
            Type::Int2 => Ok(Value::Int2(r.int2()?)),
            Type::Struct { name, member_types } => {
                let mut members = HashMap::new();

                for (member_name, member_type) in member_types {
                    let member = read_value(r, member_type)?;

                    members.insert(member_name.clone(), member);
                }

                Ok(Value::Struct {
                    name: name.clone(),
                    members,
                })
            }
            Type::ValueNotComputed => Ok(Value::ValueNotComputed),
        }
    }
}

mod write {
    use std::io::Write;

    use indexmap::indexset;

    use crate::write::{
        writable::{write_body_chunks, WriteBody},
        writer::{IdStateMut, NodeStateMut},
        BodyChunk, BodyChunks, Error, Writer,
    };

    use super::{ArrayValue, TraitsMetadata, Type, Value};

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
            [BodyChunk::normal(0, Self::write_chunk_0)].into_iter()
        }
    }

    impl TraitsMetadata {
        fn write_chunk_0<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(6)?;

            let mut types = indexset![];
            let mut type_indices = vec![];

            for t in self.traits.values() {
                let ty = value_type(t);
                let (index, _) = types.insert_full(ty);
                type_indices.push(index);
            }

            write_len(w, types.len())?;

            for ty in types {
                write_type(w, &ty)?;
            }

            write_len(w, self.traits.len())?;

            for (i, (name, t)) in self.traits.iter().enumerate() {
                let type_index = type_indices[i];

                write_len(w, name.len())?;
                w.bytes(name.as_bytes())?;
                write_len(w, type_index)?;
                write_value(w, t)?;
            }

            Ok(())
        }
    }

    fn write_len<I, N>(w: &mut Writer<impl Write, I, N>, len: usize) -> Result<(), Error> {
        if len > 127 {
            w.u8((len & 127) as u8)?;
            w.u16((len >> 7) as u16)?;
        } else {
            w.u8(len as u8)?;
        }

        Ok(())
    }

    fn write_type<I, N>(w: &mut Writer<impl Write, I, N>, ty: &Type) -> Result<(), Error> {
        match ty {
            Type::Void => w.u8(0)?,
            Type::Boolean => w.u8(1)?,
            Type::Integer => w.u8(2)?,
            Type::Real => w.u8(3)?,
            Type::Class => w.u8(4)?,
            Type::Text => w.u8(5)?,
            Type::Enum => w.u8(6)?,
            Type::Array {
                key_type,
                value_type,
            } => {
                w.u8(7)?;
                write_type(w, key_type)?;
                write_type(w, value_type)?;
            }
            Type::ParamArray => w.u8(8)?,
            Type::Vec2 => w.u8(9)?,
            Type::Vec3 => w.u8(10)?,
            Type::Int3 => w.u8(11)?,
            Type::Iso4 => w.u8(12)?,
            Type::Ident => w.u8(13)?,
            Type::Int2 => w.u8(14)?,
            Type::Struct { name, member_types } => {
                w.u8(15)?;
                w.u8(member_types.len() as u8)?;
                w.string(name)?;

                for (name, ty) in member_types {
                    w.string(name)?;
                    write_type(w, ty)?;
                }
            }
            Type::ValueNotComputed => w.u8(16)?,
        }

        Ok(())
    }

    fn write_value<I, N>(w: &mut Writer<impl Write, I, N>, value: &Value) -> Result<(), Error> {
        match *value {
            Value::Array(ArrayValue {
                ref keys,
                ref values,
                ..
            }) => {
                write_len(w, values.len())?;

                for i in 0..values.len() {
                    write_value(w, &keys[i])?;
                    write_value(w, &values[i])?;
                }
            }
            Value::Boolean(value) => w.bool8(value)?,
            Value::Class => {}
            Value::Enum => {}
            Value::Ident => {}
            Value::Int2(value) => w.int2(value)?,
            Value::Int3(value) => w.int3(value)?,
            Value::Integer(value) => w.i32(value)?,
            Value::Iso4(value) => w.iso4(value)?,
            Value::ParamArray => {}
            Value::Real(value) => w.f32(value)?,
            Value::Struct { ref members, .. } => {
                for value in members.values() {
                    write_value(w, value)?;
                }
            }
            Value::Text(ref text) => {
                write_len(w, text.len())?;
                w.bytes(text.as_bytes())?;
            }
            Value::ValueNotComputed => {}
            Value::Vec2(value) => w.vec2(value)?,
            Value::Vec3(value) => w.vec3(value)?,
            Value::Void => {}
        }

        Ok(())
    }

    fn value_type(value: &Value) -> Type {
        match value {
            Value::Array(ArrayValue {
                key_type,
                value_type,
                ..
            }) => Type::Array {
                key_type: Box::new(key_type.clone()),
                value_type: Box::new(value_type.clone()),
            },
            Value::Boolean(_) => Type::Boolean,
            Value::Class => Type::Class,
            Value::Enum => Type::Enum,
            Value::Ident => Type::Ident,
            Value::Int2(_) => Type::Int2,
            Value::Int3(_) => Type::Int3,
            Value::Integer(_) => Type::Integer,
            Value::Iso4(_) => Type::Iso4,
            Value::ParamArray => Type::ParamArray,
            Value::Real(_) => Type::Real,
            Value::Struct { name, members } => Type::Struct {
                name: name.clone(),
                member_types: members
                    .iter()
                    .map(|(name, value)| (name.clone(), value_type(value)))
                    .collect(),
            },
            Value::Text(_) => Type::Text,
            Value::ValueNotComputed => Type::ValueNotComputed,
            Value::Vec2(_) => Type::Vec2,
            Value::Vec3(_) => Type::Vec3,
            Value::Void => Type::Void,
        }
    }
}
