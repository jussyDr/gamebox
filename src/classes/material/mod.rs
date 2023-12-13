use crate::class::Class;

mod read;

/// Corresponds to Material.Gbx.
#[derive(Default)]
pub struct Material;

impl Class for Material {
    const CLASS_ID: u32 = 0x09079000;
}
