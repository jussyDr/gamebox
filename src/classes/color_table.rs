//! Types used for reading [ColorTable] nodes.

use std::io::{Read, Seek};

use serde_jsonrc::Value;

use crate::{
    read::{
        readable::{read_json, ReadJson, Sealed},
        BodyOptions, Error, HeaderOptions, Readable, Result,
    },
    Rgb,
};

/// Node type corresponding to GameBox files with the extension `ColorTable.Gbx.json`.
pub struct ColorTable {
    colors: Vec<Rgb>,
}

impl ColorTable {
    /// Colors in this color table.
    pub fn colors(&self) -> &[Rgb] {
        &self.colors
    }
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
        let colors = json
            .get("Colors")
            .ok_or("expected `Colors`")?
            .as_array()
            .ok_or("expected array")?
            .iter()
            .map(|color| {
                let color = color.as_str().ok_or::<Error>("".into())?;

                read_hex_color(color)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(ColorTable { colors })
    }
}

fn read_hex_color(s: &str) -> Result<Rgb> {
    match s.as_bytes() {
        &[b'#', r1, r2, g1, g2, b1, b2, b'0', b'0'] => {
            let r = parse_hex_byte(&[r1, r2])?;
            let g = parse_hex_byte(&[g1, g2])?;
            let b = parse_hex_byte(&[b1, b2])?;

            Ok(Rgb { r, g, b })
        }
        _ => Err("expected hex color".into()),
    }
}

fn parse_hex_byte(bytes: &[u8; 2]) -> Result<u8> {
    let mut x;

    match bytes[0] {
        b'0'..=b'9' => x = (bytes[0] - b'0') << 4,
        b'a'..=b'f' => x = (bytes[0] - b'a' + 10) << 4,
        _ => return Err("expected hex byte".into()),
    }

    match bytes[1] {
        b'0'..=b'9' => x += bytes[1] - b'0',
        b'a'..=b'f' => x += bytes[1] - b'a' + 10,
        _ => return Err("expected hex byte".into()),
    }

    Ok(x)
}
