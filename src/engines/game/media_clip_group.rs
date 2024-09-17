use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunks},
    IdStateMut,
};

/// A media clip group.
#[derive(Default)]
pub struct MediaClipGroup;

impl BodyChunks for MediaClipGroup {
    fn body_chunks<R, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        [].into_iter()
    }
}
