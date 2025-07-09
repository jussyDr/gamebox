//! Poly line 3.

use crate::ClassId;

/// Poly line 3.
#[derive(Default)]
pub struct PolyLine3;

impl ClassId for PolyLine3 {
    const CLASS_ID: u32 = 0x09118000;
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::plug::poly_line3::PolyLine3,
        read::{
            BodyChunk, BodyChunks, BodyReader, Error, ReadBody, error_unknown_chunk_version,
            read_body_chunks,
        },
    };

    impl ReadBody for PolyLine3 {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for PolyLine3 {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl PolyLine3 {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 8 {
                return Err(error_unknown_chunk_version(version));
            }

            let _poss = r.list(|r| r.vec3())?;
            let _lefts = r.list(|r| r.vec3())?;
            r.bool32()?;
            r.bool32()?;
            r.bool32()?;
            r.bool32()?;
            r.u8()?;
            r.u8()?;
            let _: Option<Arc<str>> = r.id()?;

            Ok(())
        }
    }
}
