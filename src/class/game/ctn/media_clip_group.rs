//! Media clip group.

use crate::ClassId;

/// Media clip group.
#[derive(Default)]
pub struct MediaClipGroup;

impl ClassId for MediaClipGroup {
    const CLASS_ID: u32 = 0x0307a000;
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::game::ctn::{media_clip::MediaClip, media_clip_group::MediaClipGroup},
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for MediaClipGroup {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaClipGroup {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(3, Self::read_chunk_3)]
        }
    }

    impl MediaClipGroup {
        fn read_chunk_3(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _clips: Vec<Arc<MediaClip>> = r.list_with_version(|r| r.node_ref())?;
            let _triggers = r.list(|r| {
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                let _condition = r.u32()?;
                let _condition_value = r.f32()?;
                let _coords = r.list(|r| r.uvec3())?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
