//! Types used for reading [TraitsMetadata] nodes.

use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    read::Reader,
    read::{readable::ReadBody, Result},
};

/// Traits metadata.
#[derive(Default, Debug)]
pub struct TraitsMetadata {
    traits: Vec<(String, Value)>,
}

impl Class for TraitsMetadata {
    const CLASS_ID: ClassId = ClassId::new(EngineId::SCRIPT, 2);
}

impl<R: Read, I, N> ReadBody<R, I, N> for TraitsMetadata {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 6
        let num_types = r.u8()?;
        let types = r.repeat(num_types as usize, |r| {
            let ty = Type::read(r)?;

            Ok(ty)
        })?;
        let n = r.u8()?;
        self.traits = r.repeat(n as usize, |r| {
            let size = r.u8()?;
            let name = r.string_of_len(size as usize)?;
            let type_index = r.u8()?;
            let ty = types.get(type_index as usize).ok_or("")?;

            let value = read_value(r, ty)?;

            Ok((name, value))
        })?;

        if r.u32()? != 0xfacade01 {
            return Err("expected end of node".into());
        }

        Ok(())
    }
}

fn read_value<R: Read, I, N>(r: &mut Reader<R, I, N>, ty: &Type) -> Result<Value> {
    let value = match ty {
        Type::Void => Value::Void,
        Type::Boolean => Value::Boolean(r.bool8()?),
        Type::Integer => Value::Integer(r.i32()?),
        Type::Real => Value::Real(r.f32()?),
        Type::Text => {
            let len = r.u8()?;
            Value::Text(r.string_of_len(len as usize)?)
        }
        Type::Array {
            key_type,
            element_type,
        } => {
            let len = r.u8()?;
            let array = r.repeat(len as usize, |r| {
                let key = read_value(r, key_type)?;
                let value = read_value(r, element_type)?;

                Ok((key, value))
            })?;
            Value::Array(array)
        }
    };

    Ok(value)
}

enum Type {
    Void,
    Boolean,
    Integer,
    Real,
    Text,
    Array {
        key_type: Box<Type>,
        element_type: Box<Type>,
    },
}

impl Type {
    fn read<R: Read, I, N>(r: &mut Reader<R, I, N>) -> Result<Self> {
        let ty = r.u8()?;

        match ty {
            0 => Ok(Type::Void),
            1 => Ok(Type::Boolean),
            2 => Ok(Type::Integer),
            3 => Ok(Type::Real),
            5 => Ok(Type::Text),
            7 => {
                let key_type = Self::read(r)?;
                let element_type = Self::read(r)?;

                Ok(Type::Array {
                    key_type: Box::new(key_type),
                    element_type: Box::new(element_type),
                })
            }
            _ => Err("unknown type".into()),
        }
    }
}

/// A script value.
#[derive(Debug)]
pub enum Value {
    /// Void value.
    Void,
    /// Boolean value.
    Boolean(bool),
    /// Integer value.
    Integer(i32),
    /// Real value.
    Real(f32),
    /// Text value.
    Text(String),
    /// Array value.
    Array(Vec<(Value, Value)>),
}
