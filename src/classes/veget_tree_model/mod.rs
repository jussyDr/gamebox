use crate::{class::Class, EngineId};

mod read;

#[derive(Default)]
pub struct VegetTreeModel;

impl Class for VegetTreeModel {
    const ENGINE: u8 = EngineId::META;
    const CLASS: u16 = 0x086;
}
