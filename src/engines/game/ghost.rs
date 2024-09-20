use crate::read::readable::{BodyChunk, BodyChunks};

/// A ghost.
#[derive(Default)]
pub struct Ghost;

impl BodyChunks for Ghost {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        [].into_iter()
    }
}
