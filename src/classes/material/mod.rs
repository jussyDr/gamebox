//! Types used for reading [Material] nodes.

use std::path::PathBuf;

use crate::class::Class;

mod read;

/// Node type corresponding to GameBox files with the extension `Material.Gbx`.
#[derive(Default)]
pub struct Material {
    texture_refs: Vec<PathBuf>,
}

impl Material {
    pub fn texture_refs(&self) -> &[PathBuf] {
        &self.texture_refs
    }
}

impl Class for Material {
    const ENGINE: u8 = 0x09;
    const CLASS: u16 = 0x079;
}

#[derive(Default)]
struct MaterialCustom {
    texture_refs: Vec<PathBuf>,
}

impl Class for MaterialCustom {
    const ENGINE: u8 = 0x09;
    const CLASS: u16 = 0x03a;
}
