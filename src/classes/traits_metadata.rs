use std::io::Read;

use crate::{
    common::{Class, ClassId, EngineId},
    deserialize::Deserializer,
    read::{readable::ReadBody, Result},
};

#[derive(Default)]
pub struct TraitsMetadata;

impl Class for TraitsMetadata {
    const CLASS_ID: ClassId = ClassId::new(EngineId::SCRIPT, 2);
}

impl<R: Read, I, N> ReadBody<R, I, N> for TraitsMetadata {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 6
        let num_types = d.u8()?;
        let types = d.repeat(num_types as usize, |d| {
            let ty = read_type(d)?;

            Ok(ty)
        })?;
        let n = d.u8()?;
        d.repeat(n as usize, |d| {
            let size = d.u8()?;
            d.bytes(size as usize)?;
            let type_index = d.u8()?;
            let ty = types.get(type_index as usize).ok_or("")?;

            read_value(d, ty)?;

            Ok(())
        })?;

        if d.u32()? != 0xfacade01 {
            return Err("expected end of node".into());
        }

        Ok(())
    }
}

fn read_type<R: Read, I, N>(d: &mut Deserializer<R, I, N>) -> Result<Type> {
    let ty = d.u8()?;

    match ty {
        0 => Ok(Type::Void),
        1 => Ok(Type::Boolean),
        2 => Ok(Type::Integer),
        3 => Ok(Type::Real),
        5 => Ok(Type::Text),
        7 => {
            let key_type = read_type(d)?;
            let element_type = read_type(d)?;

            Ok(Type::Array {
                key_type: Box::new(key_type),
                element_type: Box::new(element_type),
            })
        }
        _ => Err("unknown script type".into()),
    }
}

fn read_value<R: Read, I, N>(d: &mut Deserializer<R, I, N>, ty: &Type) -> Result<()> {
    match ty {
        Type::Void => {}
        Type::Boolean => {
            d.bool8()?;
        }
        Type::Integer => {
            d.i32()?;
        }
        Type::Real => {
            d.f32()?;
        }
        Type::Text => {
            let len = d.u8()?;
            d.bytes(len as usize)?;
        }
        Type::Array {
            key_type,
            element_type,
        } => {
            let len = d.u8()?;
            d.repeat(len as usize, |d| {
                read_value(d, key_type)?;
                read_value(d, element_type)?;

                Ok(())
            })?;
        }
    }

    Ok(())
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
