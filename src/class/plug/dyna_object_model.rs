use crate::read::{Error, NodeRefs, Readable};

pub struct DynaObjectModel;

impl Readable for DynaObjectModel {
    fn read(header_data: Vec<u8>, node_refs: NodeRefs, body_data: Vec<u8>) -> Result<Self, Error> {
        Ok(Self)
    }
}
