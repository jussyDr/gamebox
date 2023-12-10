//! Types for `Item`.

mod read;
mod write;

use std::ops::{Deref, DerefMut};

use crate::class::Class;

use super::collector::Collector;

/// Type corresponding to the file extension `Item.Gbx`.
pub struct Item {
    parent: Collector,
}

impl Class for Item {
    const CLASS_ID: u32 = 0x2e002000;
}

impl Deref for Item {
    type Target = Collector;

    fn deref(&self) -> &Collector {
        &self.parent
    }
}

impl DerefMut for Item {
    fn deref_mut(&mut self) -> &mut Collector {
        &mut self.parent
    }
}
