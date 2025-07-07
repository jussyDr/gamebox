//! Road chunk.

use crate::ClassId;

/// A road chunk.
#[derive(Default)]
pub struct RoadChunk;

impl ClassId for RoadChunk {
    const CLASS_ID: u32 = 0x09128000;
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::plug::road_chunk::RoadChunk,
        read::{BodyChunk, BodyChunks, Error, error_unknown_chunk_version, reader::BodyReader},
    };

    impl BodyChunks for RoadChunk {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl RoadChunk {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 12 {
                return Err(error_unknown_chunk_version(version));
            }

            r.u32()?;
            r.u32()?;
            r.list(|r| r.vec3())?;
            r.list(|r| r.vec3())?;
            r.list(|r| r.vec3())?;
            r.u32()?;
            r.list(|r| r.vec3())?;
            r.u32()?;
            r.u8()?;
            r.u8()?;
            r.f32()?;
            r.f32()?;
            r.u8()?;
            let _: Option<Arc<str>> = r.id()?;
            r.list(|r| r.u32())?;
            r.u8()?;
            let _: Option<Arc<str>> = r.id()?;
            r.vec3()?;
            r.f32()?;

            Ok(())
        }
    }
}
