use std::io::Read;

use crate::{
    read::{
        readable::{BodyChunk, BodyChunks},
        Reader,
    },
    Error,
};

/// A triangles media block.
#[derive(Default)]
pub struct MediaBlockTriangles;

impl BodyChunks for MediaBlockTriangles {
    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(1, |n, r| Self::read_chunk_1(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaBlockTriangles {
    fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        todo!()
    }
}
