//! Block info classic.

use std::ops::{Deref, DerefMut};

use crate::{ClassId, SubExtensions, class::game::ctn::block_info::BlockInfo};

/// Block info classic.
#[derive(Default)]
pub struct BlockInfoClassic {
    parent: BlockInfo,
}

impl ClassId for BlockInfoClassic {
    const CLASS_ID: u32 = 0x03051000;
}

impl Deref for BlockInfoClassic {
    type Target = BlockInfo;

    fn deref(&self) -> &BlockInfo {
        &self.parent
    }
}

impl DerefMut for BlockInfoClassic {
    fn deref_mut(&mut self) -> &mut BlockInfo {
        &mut self.parent
    }
}

impl SubExtensions for BlockInfoClassic {
    const SUB_EXTENSIONS: &[&str] = &["EDClassic"];
}

mod read {
    use crate::{
        class::game::ctn::block_info_classic::BlockInfoClassic,
        read::{
            BodyChunk, BodyChunks, BodyReader, Error, HeaderChunk, HeaderChunks, HeaderReader,
            ReadBody, Readable, read_body_chunks,
        },
    };

    impl Readable for BlockInfoClassic {}

    impl HeaderChunks for BlockInfoClassic {
        fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>> {
            []
        }
    }

    impl ReadBody for BlockInfoClassic {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for BlockInfoClassic {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            []
        }
    }
}
