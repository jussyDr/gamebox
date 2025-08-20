use crate::read::{BodyReader, ReadNode, Result};

pub struct CharPhySpecialProperty;

impl ReadNode for CharPhySpecialProperty {
    const CLASS_ID: u32 = 0x090f2000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        todo!()
    }
}
