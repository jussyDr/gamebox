//! Curve simple nod.

use crate::Class;

/// Curve simple nod.
#[derive(Default)]
pub struct CurveSimpleNod;

impl Class for CurveSimpleNod {
    const CLASS_ID: u32 = 0x09185000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        readable,
        reader::{IdStateMut, NodeStateMut, Reader},
        Error,
    };

    use self::readable::{read_body_chunks, BodyChunk, BodyChunks, ReadBody};

    use super::CurveSimpleNod;

    impl ReadBody for CurveSimpleNod {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for CurveSimpleNod {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl CurveSimpleNod {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u16()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;

            Ok(())
        }
    }
}
