//! Types used for reading [Ghost] nodes.

use crate::{class::Class, EngineId};

mod read;

/// Node type corresponding to GameBox files with the extension `Ghost.Gbx`.
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

#[derive(Default)]
pub(crate) struct EntRecordData;

impl Class for EntRecordData {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x11f;
}
