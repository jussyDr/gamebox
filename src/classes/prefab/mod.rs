use crate::{class::Class, EngineId};

mod read;

#[derive(Default)]
pub struct Prefab;

impl Class for Prefab {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x145;
}
