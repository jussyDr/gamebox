use std::path::PathBuf;

use crate::class::Class;

mod read;

#[derive(Default)]
pub struct Texture {
    image_path: PathBuf,
}

impl Class for Texture {
    const CLASS_ID: u32 = 0x09011000;
}
