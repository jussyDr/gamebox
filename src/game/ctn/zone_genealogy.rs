//! Zone genealogy.

use crate::Class;

/// A zone genealogy.
#[derive(Default)]
pub struct ZoneGenealogy;

impl Class for ZoneGenealogy {
    const CLASS_ID: u32 = 0x0311d000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::ZoneGenealogy;

    impl ReadBody for ZoneGenealogy {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for ZoneGenealogy {
        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(2, Self::read_chunk_2)].into_iter()
        }
    }

    impl ZoneGenealogy {
        fn read_chunk_2<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let _zone_ids = r.list(|r| r.id())?;
            let _current_index = r.u32()?;
            let _dir = r.u32()?;
            let _current_zone_id = r.id()?;

            Ok(())
        }
    }
}
