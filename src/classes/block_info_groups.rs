use std::io::Read;

use serde_jsonrc::{Map, Value};

use crate::read::{
    readable::{read_json, ReadJson, Sealed},
    BodyOptions, Error, HeaderOptions, Readable, Result,
};

pub struct BlockInfoGroups {
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

impl ReadJson for BlockInfoGroups {
    const CLASS_NAME: &'static str = "CGameBlockInfoGroups";

    fn read(json: &Map<String, Value>) -> Result<Self> {
        let groups = json
            .get("Groups")
            .ok_or("m")?
            .as_array()
            .ok_or("f")?
            .iter()
            .map(|value| {
                let group_id = value
                    .get("GroupId")
                    .ok_or("    q")?
                    .as_str()
                    .ok_or("d")?
                    .to_owned();

                let block_ids = if let Some(value) = value.get("BlockIds") {
                    value
                        .as_array()
                        .ok_or("a")?
                        .iter()
                        .map(|value| {
                            let block_id = value.as_str().ok_or("c")?.to_owned();

                            Ok::<String, Error>(block_id)
                        })
                        .collect::<Result<Vec<_>>>()?
                } else {
                    vec![]
                };

                Ok::<BlockInfoGroup, Error>(BlockInfoGroup {
                    id: group_id,
                    block_ids,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(BlockInfoGroups { groups })
    }
}

pub struct BlockInfoGroup {
    id: String,
    block_ids: Vec<String>,
}
