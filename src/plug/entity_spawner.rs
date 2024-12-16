//! Entity spawner.

use crate::Class;

/// Entity spawner.
#[derive(Default)]
pub struct EntitySpawner;

impl Class for EntitySpawner {
    const CLASS_ID: u32 = 0x09123000;
}

mod read {
    use crate::read::{reader::Reader, Error, ReadBody};

    use super::EntitySpawner;

    impl ReadBody for EntitySpawner {
        fn read_body<R, I, N>(&mut self, _: &mut Reader<R, I, N>) -> Result<(), Error> {
            Ok(())
        }
    }
}
