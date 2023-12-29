//! Types used for reading [Texture] nodes.

mod read;

use std::path::Path;

use crate::{class::Class, EngineId, RcPath};

/// Node type corresponding to GameBox files with the extension `Texture.Gbx`.
#[derive(Default)]
pub struct Texture {
    image_path: RcPath,
}

impl Texture {
    pub fn image_path(&self) -> &Path {
        &self.image_path
    }
}

impl Class for Texture {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x011;
}
