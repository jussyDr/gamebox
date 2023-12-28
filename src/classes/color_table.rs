//! Types used for reading [ColorTable] nodes.

use std::io::{Read, Seek};

use serde_jsonrc::Value;

use crate::{
    read::{
        readable::{read_json, ReadJson, Sealed},
        BodyOptions, HeaderOptions, Readable, Result,
    },
    Rgb,
};

/// Node type corresponding to GameBox files with the extension `ColorTable.Gbx.json`.
pub struct ColorTable {
    colors: Vec<Rgb>,
}

impl Readable for ColorTable {}

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
        let colors = colors
            .iter()
            .map(|color| read_hex_color(color.as_str().unwrap()).unwrap())
            .collect::<Vec<_>>();

        Ok(ColorTable { colors })
    }
}

fn read_hex_color(s: &str) -> Result<Rgb> {
    let bytes = s.as_bytes();

    if bytes.len() != 9 {
        todo!()
    }

    if bytes[0] != b'#' {
        todo!()
    }

    let mut r = 0;

    match bytes[1] {
        b'0'..=b'9' => r = (bytes[1] - b'0') << 4,
        b'a'..=b'f' => r = (bytes[1] - b'a' + 10) << 4,
        _ => todo!(),
    }

    match bytes[2] {
        b'0'..=b'9' => r += bytes[2] - b'0',
        b'a'..=b'f' => r += bytes[2] - b'a' + 10,
        _ => todo!(),
    }

    let mut g = 0;

    match bytes[3] {
        b'0'..=b'9' => g = (bytes[3] - b'0') << 4,
        b'a'..=b'f' => g = (bytes[3] - b'a' + 10) << 4,
        _ => todo!(),
    }

    match bytes[4] {
        b'0'..=b'9' => g += bytes[4] - b'0',
        b'a'..=b'f' => g += bytes[4] - b'a' + 10,
        _ => todo!(),
    }

    let mut b = 0;

    match bytes[5] {
        b'0'..=b'9' => b = (bytes[5] - b'0') << 4,
        b'a'..=b'f' => b = (bytes[5] - b'a' + 10) << 4,
        _ => todo!(),
    }

    match bytes[6] {
        b'0'..=b'9' => b += bytes[6] - b'0',
        b'a'..=b'f' => b += bytes[6] - b'a' + 10,
        _ => todo!(),
    }

    if bytes[7] != b'0' && bytes[8] != b'0' {
        todo!()
    }

    Ok(Rgb { r, g, b })
}
