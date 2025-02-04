//! Collector list.

use crate::Class;

/// Collector list.
#[derive(PartialEq, Eq, Hash, Default)]
pub struct CollectorList {
    list: Vec<()>,
}

impl Class for CollectorList {
    const CLASS_ID: u32 = 0x0301b000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::CollectorList;

    impl ReadBody for CollectorList {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for CollectorList {
        fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>>
        {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl CollectorList {
        fn read_chunk_0<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            self.list = r.list(|r| {
                let _block_model_id = r.id()?;
                let _block_model_collection_id = r.id()?;
                let _block_model_author_id = r.id()?;
                let _count = r.u32()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}

mod write {
    use std::io::Write;

    use crate::write::{
        writable::{write_body_chunks, WriteBody},
        writer::{IdStateMut, NodeStateMut},
        BodyChunk, BodyChunks, Error, Writer,
    };

    use super::CollectorList;

    impl WriteBody for CollectorList {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for CollectorList {
        fn body_chunks<W: Write, I, N>() -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [BodyChunk::normal(0, Self::write_chunk_0)].into_iter()
        }
    }

    impl CollectorList {
        fn write_chunk_0<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0)?;

            Ok(())
        }
    }
}
