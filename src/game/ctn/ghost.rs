//! Ghost.

use crate::Class;

/// A Ghost.
#[derive(Default)]
pub struct Ghost {
    parent: crate::game::ghost::Ghost,
}

impl Class for Ghost {
    const CLASS_ID: u32 = 0x03092000;
}

mod read {
    use std::io::Read;

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::Ghost;

    impl ReadBody for Ghost {
        fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Ghost {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }
}
