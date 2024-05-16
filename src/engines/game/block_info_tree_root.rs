use std::io::Read;

use serde::Deserialize;

use crate::read::{
    json::{read_json, ClassName},
    readable::Sealed,
    BodyOptions, HeaderOptions, Readable, Result,
};

/// Block info tree root.
#[derive(Deserialize)]
pub struct BlockInfoTreeRoot {}

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

impl ClassName for BlockInfoTreeRoot {
    const CLASS_NAME: &'static str = "CGameBlockInfoTreeRoot";
}
