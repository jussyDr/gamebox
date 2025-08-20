use crate::read::{BodyReader, ReadNode, Result};

pub struct Ghost;

impl ReadNode for Ghost {
    const CLASS_ID: u32 = 0x03092000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        todo!()
    }
}
