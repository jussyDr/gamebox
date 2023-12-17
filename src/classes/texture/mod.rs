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
    const ENGINE: u8 = 0x09;
    const CLASS: u16 = 0x011;
}
