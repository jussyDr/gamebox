use crate::class::Class;

mod read;

#[derive(Default)]
pub struct VegetTreeModel;

impl Class for VegetTreeModel {
    const ENGINE: u8 = 0x2f;
    const CLASS: u16 = 0x086;
}
