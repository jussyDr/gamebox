//! Types for `Item`.

mod read;
mod write;

use crate::class::Class;

/// Type corresponding to the file extension `Item.Gbx`.
pub struct Item;

impl Class for Item {
    const CLASS_ID: u32 = 0x2e002000;
}
