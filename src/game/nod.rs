//! Nod.

use crate::Class;

/// Nod.
#[derive(Default)]
pub struct Nod;

impl Class for Nod {
    const CLASS_ID: u32 = 0x03008000;
}

mod read {
    use std::io::Read;

    use crate::read::{
        reader::{IdStateMut, Reader},
        BodyChunk, BodyChunks, Error,
    };

    use super::Nod;

    impl BodyChunks for Nod {
        fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>>
        {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl Nod {
        fn read_chunk_0<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            r.id_or_null()?;

            Ok(())
        }
    }
}
