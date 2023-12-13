use crate::class::Class;

mod read;

#[derive(Default)]
pub struct Texture;

impl Class for Texture {
    const CLASS_ID: u32 = 0x09011000;
}
