//! Poly line 3.

use crate::Class;

/// Poly line 3
#[derive(Default)]
pub struct PolyLine3;

impl Class for PolyLine3 {
    const CLASS_ID: u32 = 0x09118000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::PolyLine3;

    impl ReadBody for PolyLine3 {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for PolyLine3 {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl PolyLine3 {
        fn read_chunk_0<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 8 {
                return Err(Error::chunk_version(version));
            }

            let _poss = r.list(|r| r.vec3::<f32>())?;
            let _lefts = r.list(|r| r.vec3::<f32>())?;
            r.bool()?;
            r.bool()?;
            r.bool()?;
            r.u8()?;
            r.u8()?;
            r.u32()?;
            r.id_or_null()?;

            Ok(())
        }
    }
}
