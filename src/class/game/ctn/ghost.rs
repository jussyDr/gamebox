//! Ghost.

use crate::ClassId;

/// Ghost.
#[derive(Default)]
pub struct Ghost;

impl ClassId for Ghost {
    const CLASS_ID: u32 = 0x03092000;
}

mod read {
    use crate::{
        class::game::ctn::ghost::Ghost,
        read::{Error, ReadBody, reader::BodyReader},
    };

    impl ReadBody for Ghost {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            todo!()
        }
    }
}
