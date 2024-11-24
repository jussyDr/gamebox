use crate::Class;

/// A collector list.
#[derive(Default)]
pub struct CollectorList;

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
            [BodyChunk::new(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl CollectorList {
        fn read_chunk_0<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let _collector_stock = r.list(|r| {
                let _block_model = r.id()?;
                let _block_model = r.id()?;
                let _block_model = r.id()?;
                let _count = r.u32()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
