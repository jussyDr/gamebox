//! Media clip list.

use crate::Class;

/// Media clip list.
#[derive(Default)]
pub struct MediaClipList;

impl Class for MediaClipList {
    const CLASS_ID: u32 = 0x09189000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::MediaClipList;

    impl ReadBody for MediaClipList {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MediaClipList {
        fn body_chunks<R: Read, I, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl MediaClipList {
        fn read_chunk_0<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.list(|r| r.external_node_ref::<()>())?;

            Ok(())
        }
    }
}
