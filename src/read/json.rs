use std::io::Read;

use serde::Deserialize;
use serde_json_lenient::{Map, Value};

use super::Result;

pub trait ClassName {
    const CLASS_NAME: &'static str;
}

pub fn read_json<'de, T: ClassName + Deserialize<'de>>(reader: impl Read) -> Result<T> {
    let mut de = serde_json_lenient::Deserializer::from_reader(reader);
    de.set_allow_comments(false);

    let mut object: Map<String, Value> = Deserialize::deserialize(&mut de)?;

    de.end()?;

    let class_name = object.remove("ClassId").ok_or("")?;

    if class_name != T::CLASS_NAME {
        return Err("".into());
    }

    let node: T = Deserialize::deserialize(Value::Object(object))?;

    Ok(node)
}
