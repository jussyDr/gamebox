//! Entity record data.

use crate::Class;

/// Entity record data.
#[derive(Default)]
pub struct EntRecordData;

impl Class for EntRecordData {
    const CLASS_ID: u32 = 0x0911f000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::EntRecordData;

    impl ReadBody for EntRecordData {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for EntRecordData {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl EntRecordData {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 10 {
                return Err(Error::chunk_version(version));
            }

            let _size = r.u32()?;
            let _data = r.byte_buf()?;

            Ok(())
        }
    }
}
