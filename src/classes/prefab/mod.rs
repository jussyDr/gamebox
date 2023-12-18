use crate::class::Class;

mod read;

#[derive(Default)]
pub struct Prefab;

impl Class for Prefab {
    const ENGINE: u8 = 0x09;
    const CLASS: u16 = 0x145;
}
