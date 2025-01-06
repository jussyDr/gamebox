//! Zone flat.

use std::ops::Deref;

use super::Zone;

/// Zone flat.
pub struct ZoneFlat {
    parent: Zone,
}

impl Deref for ZoneFlat {
    type Target = Zone;

    fn deref(&self) -> &Zone {
        &self.parent
    }
}
