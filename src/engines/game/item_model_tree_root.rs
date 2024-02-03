use std::io::Read;

use serde::Deserialize;

use crate::read::{
    json::{read_json, ClassName},
    readable::Sealed,
    BodyOptions, HeaderOptions, Readable, Result,
};

#[derive(Deserialize)]
pub struct ItemModelTreeRoot {}

impl Readable for ItemModelTreeRoot {}

impl Sealed for ItemModelTreeRoot {
    fn read(
        reader: impl Read,
        _header_options: HeaderOptions,
        _body_options: BodyOptions,
    ) -> Result<Self> {
        read_json(reader)
    }
}

impl ClassName for ItemModelTreeRoot {
    const CLASS_NAME: &'static str = "CGameItemModelTreeRoot";
}
