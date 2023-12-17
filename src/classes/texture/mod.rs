//! Types used for reading [Texture] nodes.

use std::path::PathBuf;

use crate::class::Class;

mod read;

/// Node type corresponding to GameBox files with the extension `Texture.Gbx`.
#[derive(Default)]
pub struct Texture {
    image_path: PathBuf,
}

impl Class for Texture {
    const CLASS_ID: u32 = 0x09011000;
}
