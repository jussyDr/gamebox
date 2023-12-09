//! Types for `Item`.

mod read;
mod write;

use crate::class::Class;

use self::read::Collector;

/// Type corresponding to the file extension `Item.Gbx`.
pub struct Item {
    parent: Collector,
}

impl Class for Item {
    const CLASS_ID: u32 = 0x2e002000;
}
