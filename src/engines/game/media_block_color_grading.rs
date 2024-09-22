use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunks},
    Error, Reader,
};

/// A color grading block.
pub struct MediaBlockColorGrading;

impl BodyChunks for MediaBlockColorGrading {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 2] = [
            (0, |n, r| Self::read_chunk_0(n, r), false),
            (1, |n, r| Self::read_chunk_1(n, r), false),
        ];

        chunks.into_iter()
    }
}

impl MediaBlockColorGrading {
    fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _image = r.pack_desc()?;

        Ok(())
    }

    fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _keys = r.list(|r| {
            let _time = r.f32()?;
            let _itensity = r.f32()?;

            Ok(())
        })?;

        Ok(())
    }
}
