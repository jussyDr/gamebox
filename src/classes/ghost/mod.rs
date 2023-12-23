use crate::{class::Class, EngineId};

mod read;

#[derive(Default)]
pub struct Ghost {
    parent: Ghost2,
}

impl Class for Ghost {
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x092;
}

#[derive(Default)]
struct Ghost2;
