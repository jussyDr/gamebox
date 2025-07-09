//! Media clip list.

use crate::ClassId;

/// Media clip list.
#[derive(Default)]
pub struct MediaClipList;

impl ClassId for MediaClipList {
    const CLASS_ID: u32 = 0x09189000;
}

mod read {
    use crate::{
        ExternalNodeRef,
        class::{game::ctn::media_clip::MediaClip, plug::media_clip_list::MediaClipList},
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for MediaClipList {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaClipList {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl MediaClipList {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            let _: Vec<ExternalNodeRef<MediaClip>> = r.list(|r| r.node_ref())?;

            Ok(())
        }
    }
}
