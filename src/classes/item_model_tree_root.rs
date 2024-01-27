use std::io::Read;

use serde_jsonrc::{Map, Value};

use crate::read::{
    readable::{read_json, ReadJson, Sealed},
    BodyOptions, HeaderOptions, Readable, Result,
};

pub struct ItemModelTreeRoot;

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

impl ReadJson for ItemModelTreeRoot {
    const CLASS_NAME: &'static str = "CGameItemModelTreeRoot";

    fn read(json: &Map<String, Value>) -> Result<Self> {
        Ok(ItemModelTreeRoot)
    }
}
