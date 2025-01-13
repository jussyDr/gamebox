//! Ghost.

use crate::Class;

/// A ghost.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
pub struct Ghost;

impl Class for Ghost {
    const CLASS_ID: u32 = 0x0303f000;
}

mod read {
    use std::io::Read;

    use crate::read::{reader::Reader, BodyChunk, BodyChunks, Error};

    use super::Ghost;

    impl BodyChunks for Ghost {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(6, Self::read_chunk_6),
                BodyChunk::skippable(7, |s, r| Self::read_chunk_7(s, r)),
            ]
            .into_iter()
        }
    }

    impl Ghost {
        fn read_chunk_6<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _is_replaying = r.bool()?;
            let _size = r.u32()?;
            let _data = r.byte_buf()?;

            Ok(())
        }

        fn read_chunk_7<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }
    }
}
