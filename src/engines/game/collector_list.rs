//! CollectorList class.

use crate::read::readable::{BodyChunk, BodyChunks};

/// CollectorList class.
#[derive(Default)]
pub struct CollectorList;

impl BodyChunks for CollectorList {
    fn body_chunks<R, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        [].into_iter()
    }
}
