//! Zone genealogy.

use std::sync::Arc;

use crate::Class;

use super::Direction;

/// A zone genealogy.
#[derive(Default)]
pub struct ZoneGenealogy {
    zone_ids: Vec<Arc<str>>,
    current_index: u32,
    dir: Direction,
    current_zone_id: Option<Arc<str>>,
}

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
            self.zone_ids = r.list(|r| r.id())?;
            self.current_index = r.u32()?;
            self.dir = r.enum_u32()?;
            self.current_zone_id = r.id_or_null()?;

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

    use super::ZoneGenealogy;

    impl WriteBody for ZoneGenealogy {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for ZoneGenealogy {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [].into_iter()
        }
    }
}
