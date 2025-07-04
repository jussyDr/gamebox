//! Entity spawner.

use crate::ClassId;

/// Entity spawner.
#[derive(Default)]
pub struct EntitySpawner;

impl ClassId for EntitySpawner {
    const CLASS_ID: u32 = 0x09123000;
}
