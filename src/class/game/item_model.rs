use crate::read::{Error, NodeRefs, Readable};

pub struct ItemModel;

impl Readable for ItemModel {
    fn read(header_data: Vec<u8>, node_refs: NodeRefs, body_data: Vec<u8>) -> Result<Self, Error> {
        Ok(Self)
    }
}
