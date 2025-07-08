//! Skel.

use crate::ClassId;

/// Skel.
#[derive(Default)]
pub struct Skel;

impl ClassId for Skel {
    const CLASS_ID: u32 = 0x090ba000;
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::plug::skel::Skel,
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
    };

    impl ReadBody for Skel {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Skel {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl Skel {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 19 {
                return Err(error_unknown_chunk_version(version));
            }

            let _name: Option<Arc<str>> = r.id()?;

            let joints_length = r.u16()?;
            r.repeat(joints_length as usize, |r| {
                let _name: Arc<str> = r.id()?;
                let _parent_index = r.u16()?;
                r.iso4()?;

                Ok(())
            })?;

            if r.bool32()? {
                todo!()
            }

            let _sockets = r.list(|r| {
                let _name: Arc<str> = r.id()?;
                r.u16()?;
                r.iso4()?;

                Ok(())
            })?;

            if r.bool32()? {
                todo!()
            }

            r.byte_buf()?;
            r.byte_buf()?;
            r.byte_buf()?;
            r.u8()?;
            r.u32()?;

            Ok(())
        }
    }
}
