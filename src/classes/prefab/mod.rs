use crate::class::Class;

mod read;

#[derive(Default)]
pub struct Prefab;

impl Class for Prefab {
    const ENGINE: u8 = 0;
    const CLASS: u16 = 0;
}
