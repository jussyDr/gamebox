//! Types used for reading [Texture] nodes.

mod read;

use std::path::Path;

use crate::{class::Class, EngineId, RcPath};

/// Node type corresponding to GameBox files with the extension `Texture.Gbx`.
#[derive(Default)]
pub struct Texture {
    image_ref: RcPath,
}

impl Texture {
    pub fn image_ref(&self) -> &Path {
        &self.image_ref
    }
}

impl Class for Texture {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x011;
}
