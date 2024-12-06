//! Road chunk.

use crate::Class;

/// A road chunk.
#[derive(Default)]
pub struct RoadChunk;

impl Class for RoadChunk {
    const CLASS_ID: u32 = 0x09128000;
}

mod read {
    use std::io::Read;

    use crate::read::{
        reader::{IdStateMut, Reader},
        BodyChunk, BodyChunks, Error,
    };

    use super::RoadChunk;

    impl BodyChunks for RoadChunk {
        fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>>
        {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl RoadChunk {
        fn read_chunk_0<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.list(|r| {
                r.f32()?;
                r.f32()?;
                r.f32()?;

                Ok(())
            })?;
            r.list(|r| {
                r.f32()?;
                r.f32()?;
                r.f32()?;

                Ok(())
            })?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u16()?;
            r.u8()?;
            r.id()?;
            r.u32()?;
            r.u8()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
