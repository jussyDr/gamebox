//! Types used for reading [Texture] nodes.

use std::path::{Path, PathBuf};

use crate::class::Class;

mod read;

/// Node type corresponding to GameBox files with the extension `Texture.Gbx`.
#[derive(Default)]
pub struct Texture {
    image_ref: PathBuf,
}

impl Texture {
    pub fn image_ref(&self) -> &Path {
        &self.image_ref
    }
}

impl Class for Texture {
    const ENGINE: u8 = 0x09;
    const CLASS: u16 = 0x011;
}
