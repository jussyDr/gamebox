use crate::read::{Error, NodeRefs, Readable};

pub struct Solid2Model;

impl Readable for Solid2Model {
    fn read(header_data: Vec<u8>, node_refs: NodeRefs, body_data: Vec<u8>) -> Result<Self, Error> {
        Ok(Self)
    }
}
