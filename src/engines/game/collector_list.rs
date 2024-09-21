use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunks},
    Error, IdStateMut, Reader,
};

/// A collector list.
#[derive(Default)]
pub struct CollectorList;

impl BodyChunks for CollectorList {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(0, |n, r| Self::read_chunk_0(n, r), false)];

        chunks.into_iter()
    }
}

impl CollectorList {
    fn read_chunk_0<N>(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, N>,
    ) -> Result<(), Error> {
        let _collector_stock = r.list(|r| {
            let _block_model = r.ident()?;
            let _count = r.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}
