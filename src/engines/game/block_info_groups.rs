use std::io::Read;

use serde::Deserialize;

use crate::read::{
    json::{read_json, ClassName},
    readable::Sealed,
    BodyOptions, HeaderOptions, Readable, Result,
};

/// Block info groups.
#[derive(Deserialize, Debug)]
pub struct BlockInfoGroups {
    #[serde(rename = "Groups")]
    groups: Vec<BlockInfoGroup>,
}

impl Readable for BlockInfoGroups {}

impl Sealed for BlockInfoGroups {
    fn read(
        reader: impl Read,
        _header_options: HeaderOptions,
        _body_options: BodyOptions,
    ) -> Result<Self> {
        read_json(reader)
    }
}

impl ClassName for BlockInfoGroups {
    const CLASS_NAME: &'static str = "CGameBlockInfoGroups";
}

/// A block info group.
#[derive(Deserialize, Debug)]
pub struct BlockInfoGroup {
    #[serde(rename = "GroupId")]
    id: String,
    #[serde(rename = "BlockIds", default)]
    block_ids: Vec<String>,
}
