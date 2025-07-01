//! Challenge

use crate::{ClassId, Extensions};

/// A challenge.
#[derive(Default)]
pub struct Challenge;

impl ClassId for Challenge {
    const CLASS_ID: u32 = 0x03043000;
}

impl Extensions for Challenge {
    const EXTENSIONS: &[&str] = &["Map.Gbx"];
}

mod read {
    use std::io::Read;

    use crate::{
        class::game::challenge::Challenge,
        read::{
            BodyChunk, BodyChunks, Error, HeaderChunk, HeaderChunks, ReadBody, Readable,
            read_body_chunks,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl Readable for Challenge {}

    impl HeaderChunks for Challenge {
        fn header_chunks<R, I, N>() -> impl IntoIterator<Item = HeaderChunk<Self, R, I, N>> {
            []
        }
    }

    impl ReadBody for Challenge {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Challenge {
        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
            []
        }
    }
}
