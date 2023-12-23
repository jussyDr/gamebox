//! Types used for reading and writing [Map] nodes.

use std::rc::Rc;

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
    id: Rc<str>,
    is_free: bool,
}

impl Block {
    /// Identifier of the block.
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Item placed inside of a [Map].
#[derive(Default)]
pub struct Item {
    id: Option<Rc<str>>,
}

impl Item {
    /// Identifier of the item.
    pub fn id(&self) -> &str {
        match self.id {
            None => "",
            Some(ref id) => id,
        }
    }
}
