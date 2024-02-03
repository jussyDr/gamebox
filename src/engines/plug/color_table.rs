//! Types used for reading [ColorTable] nodes.

use std::io::Read;

use serde::Deserialize;

use crate::read::{
    json::{read_json, ClassName},
    readable::Sealed,
    BodyOptions, HeaderOptions, Readable, Result,
};

/// Node type corresponding to GameBox files with the extension `ColorTable.Gbx.json`.
#[derive(Deserialize)]
pub struct ColorTable {}

impl Readable for ColorTable {}

impl Sealed for ColorTable {
    fn read(
        reader: impl Read,
        _header_options: HeaderOptions,
        _body_options: BodyOptions,
    ) -> Result<Self> {
        read_json(reader)
    }
}

impl ClassName for ColorTable {
    const CLASS_NAME: &'static str = "CPlugMaterialColorTargetTable";
}
