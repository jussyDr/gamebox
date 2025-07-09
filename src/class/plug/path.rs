//! Path.

use crate::ClassId;

/// Path.
#[derive(Default)]
pub struct Path;

impl ClassId for Path {
    const CLASS_ID: u32 = 0x09119000;
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::plug::{path::Path, poly_line3::PolyLine3},
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
    };

    impl ReadBody for Path {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Path {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl Path {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(error_unknown_chunk_version(version));
            }

            let _poly_lines: Vec<Arc<PolyLine3>> = r.list(|r| r.node_ref())?;
            r.bool32()?;
            r.u8()?;
            let _line_groups = r.byte_buf()?;

            Ok(())
        }
    }
}
