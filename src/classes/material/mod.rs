//! Types used for reading [Material] nodes.

use crate::class::Class;

mod read;

/// Node type corresponding to GameBox files with the extension `Material.Gbx`.
#[derive(Default)]
pub struct Material;

impl Class for Material {
    const CLASS_ID: u32 = 0x09079000;
}
