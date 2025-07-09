//! Block skin.

use crate::ClassId;

/// Block skin.
#[derive(Default)]
pub struct BlockSkin;

impl ClassId for BlockSkin {
    const CLASS_ID: u32 = 0x03059000;
}

mod read {
    use crate::{
        class::game::ctn::{block_skin::BlockSkin, read_file_ref},
        read::{
            BodyChunk, BodyChunks, BodyReader, Error, ReadBody, error_unknown_chunk_version,
            read_body_chunks,
        },
    };

    impl ReadBody for BlockSkin {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for BlockSkin {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(2, Self::read_chunk_2),
                BodyChunk::new(3, Self::read_chunk_3),
            ]
        }
    }

    impl BlockSkin {
        fn read_chunk_2(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _text = r.string()?;
            let _file_ref = read_file_ref(r)?;
            let _parent_file_ref = read_file_ref(r)?;

            Ok(())
        }

        fn read_chunk_3(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            let _foreground_file_ref = read_file_ref(r)?;

            Ok(())
        }
    }
}
