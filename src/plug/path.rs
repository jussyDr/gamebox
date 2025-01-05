//! Path.

use std::sync::Arc;

use crate::Class;

use super::poly_line_3::PolyLine3;

/// Path.
#[derive(Default, Debug)]
pub struct Path {
    poly_lines: Vec<Arc<PolyLine3>>,
}

impl Class for Path {
    const CLASS_ID: u32 = 0x09119000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::Path;

    impl ReadBody for Path {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Path {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl Path {
        fn read_chunk_0(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            self.poly_lines = r.list(|r| r.internal_node_ref())?;
            r.bool()?;
            r.u8()?;
            r.byte_buf()?;

            Ok(())
        }
    }
}
