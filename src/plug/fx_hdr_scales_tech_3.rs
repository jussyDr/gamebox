//! Fx Hdr scales tech 3.

use crate::Class;

/// Fx Hdr scales tech 3.
#[derive(Default)]
pub struct FxHdrScalesTech3;

impl Class for FxHdrScalesTech3 {
    const CLASS_ID: u32 = 0x090f5000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        readable,
        reader::{IdStateMut, NodeStateMut, Reader},
        Error,
    };

    use self::readable::{read_body_chunks, BodyChunk, BodyChunks, ReadBody};

    use super::FxHdrScalesTech3;

    impl ReadBody for FxHdrScalesTech3 {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for FxHdrScalesTech3 {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl FxHdrScalesTech3 {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;

            Ok(())
        }
    }
}
