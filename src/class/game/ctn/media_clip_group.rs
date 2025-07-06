//! Media clip group.

use crate::ClassId;

/// Media clip group.
#[derive(Default)]
pub struct MediaClipGroup;

impl ClassId for MediaClipGroup {
    const CLASS_ID: u32 = 0x0307a000;
}

mod read {
    use crate::{
        class::game::ctn::media_clip_group::MediaClipGroup,
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for MediaClipGroup {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaClipGroup {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            []
        }
    }
}
