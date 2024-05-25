use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::Deserializer,
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
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 6
        let num_types = d.u8()?;
        let types = d.repeat(num_types as usize, |d| {
            let ty = Type::read(d)?;

            Ok(ty)
        })?;
        let n = d.u8()?;
        self.traits = d.repeat(n as usize, |d| {
            let size = d.u8()?;
            let name = d.string_of_len(size as usize)?;
            let type_index = d.u8()?;
            let ty = types.get(type_index as usize).ok_or("")?;

            let value = read_value(d, ty)?;

            Ok((name, value))
        })?;

        if d.u32()? != 0xfacade01 {
            return Err("expected end of node".into());
        }

        Ok(())
    }
}

fn read_value<R: Read, I, N>(d: &mut Deserializer<R, I, N>, ty: &Type) -> Result<Value> {
    let value = match ty {
        Type::Void => Value::Void,
        Type::Boolean => Value::Boolean(d.bool8()?),
        Type::Integer => Value::Integer(d.i32()?),
        Type::Real => Value::Real(d.f32()?),
        Type::Text => {
            let len = d.u8()?;
            Value::Text(d.string_of_len(len as usize)?)
        }
        Type::Array {
            key_type,
            element_type,
        } => {
            let len = d.u8()?;
            let array = d.repeat(len as usize, |d| {
                let key = read_value(d, key_type)?;
                let value = read_value(d, element_type)?;

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
    fn read<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Self> {
        let ty = d.u8()?;

        match ty {
            0 => Ok(Type::Void),
            1 => Ok(Type::Boolean),
            2 => Ok(Type::Integer),
            3 => Ok(Type::Real),
            5 => Ok(Type::Text),
            7 => {
                let key_type = Self::read(d)?;
                let element_type = Self::read(d)?;

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
