use crate::read::{BodyReader, ReadNode, Result};

pub struct VehicleCarMarksSamples;

impl ReadNode for VehicleCarMarksSamples {
    const CLASS_ID: u32 = 0x0a083000;

    fn read_node(r: &mut impl BodyReader) -> Result<Self> {
        todo!()
    }
}
