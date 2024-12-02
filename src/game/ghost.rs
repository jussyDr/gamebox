//! Ghost.

use crate::Class;

/// A ghost.
#[derive(PartialEq, Default, Debug)]
pub struct Ghost;

impl Class for Ghost {
    const CLASS_ID: u32 = 0x0303f000;
}

mod read {
    use crate::read::{BodyChunk, BodyChunks};

    use super::Ghost;

    impl BodyChunks for Ghost {
        fn body_chunks<R, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }
}
