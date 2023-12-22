//! Types used for reading and writing [Map] nodes.

use crate::{class::Class, EngineId};

mod read;

/// Node type corresponding to GameBox files with the extension `Map.Gbx`.
#[derive(Default)]
pub struct Map {
    blocks: Vec<Block>,
    baked_blocks: Vec<Block>,
    items: Vec<Item>,
}

impl Class for Map {
    const ENGINE: u8 = EngineId::GAME;
    const CLASS: u16 = 0x043;
}

impl Map {
    /// List of blocks placed inside of this map.
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// List of items placed inside of this map.
    pub fn items(&self) -> &[Item] {
        &self.items
    }
}

/// Block placed inside of a [Map].
pub struct Block {
    id: String,
    is_free: bool,
}

/// Item placed inside of a [Map].
#[derive(Default)]
pub struct Item {
    id: String,
}
