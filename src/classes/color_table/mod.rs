//! Types used for reading [ColorTable] nodes.

use std::io::{Read, Seek};

use serde_jsonrc::Value;

use crate::read::{read_json, readable::Sealed, BodyOptions, HeaderOptions, ReadJson, Result};

/// Node type corresponding to GameBox files with the extension `ColorTable.Gbx.json`.
pub struct ColorTable {
    colors: Vec<()>,
}

impl Sealed for ColorTable {
    fn read(
        reader: impl Read + Seek,
        _header_options: HeaderOptions,
        _body_options: BodyOptions,
    ) -> Result<Self> {
        read_json(reader)
    }
}

impl ReadJson for ColorTable {
    const CLASS_NAME: &'static str = "CPlugMaterialColorTargetTable";

    fn read(json: Value) -> Result<Self> {
        let colors = json.get("Colors").unwrap().as_array().unwrap();
        let colors = colors.iter().map(|color| ()).collect();

        Ok(ColorTable { colors })
    }
}
