use std::io::Read;

use serde_json::{Map, Value};

use crate::read::{
    readable::{read_json, ReadJson, Sealed},
    BodyOptions, HeaderOptions, Readable, Result,
};

pub struct BlockInfoTreeRoot;

impl Readable for BlockInfoTreeRoot {}

impl Sealed for BlockInfoTreeRoot {
    fn read(
        reader: impl Read,
        _header_options: HeaderOptions,
        _body_options: BodyOptions,
    ) -> Result<Self> {
        read_json(reader)
    }
}

impl ReadJson for BlockInfoTreeRoot {
    const CLASS_NAME: &'static str = "CGameBlockInfoTreeRoot";

    fn read(json: &Map<String, Value>) -> Result<Self> {
        Ok(BlockInfoTreeRoot)
    }
}
