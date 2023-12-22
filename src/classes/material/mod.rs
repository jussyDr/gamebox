//! Types used for reading [Material] nodes.

use std::path::{Path, PathBuf};

use crate::{class::Class, EngineId};

mod read;

/// Node type corresponding to GameBox files with the extension `Material.Gbx`.
#[derive(Default)]
pub struct Material {
    diffuse_texture_ref: PathBuf,
}

impl Material {
    pub fn diffuse_texture_ref(&self) -> &Path {
        &self.diffuse_texture_ref
    }
}

impl Class for Material {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x079;
}

#[derive(Default)]
struct MaterialCustom {
    diffuse_texture_ref: PathBuf,
}

impl Class for MaterialCustom {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x03a;
}
