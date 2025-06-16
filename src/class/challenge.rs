//! Challenge

use crate::Class;

/// Challenge
#[derive(Default)]
pub struct Challenge;

impl Class for Challenge {
    fn class_id(&self) -> u32 {
        0x03043000
    }
}

mod read {
    use std::io::Read;

    use crate::{
        class::challenge::Challenge,
        read::{
            Error, ReadBody, Readable,
            reader::{NodesMut, Reader},
        },
    };

    impl Readable for Challenge {}

    impl ReadBody for Challenge {
        fn read_body<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodesMut>,
        ) -> Result<(), Error> {
            todo!()
        }
    }
}
