//! Media block image.

use crate::ClassId;

/// Media block image.
#[derive(Default)]
pub struct MediaBlockImage;

impl ClassId for MediaBlockImage {
    const CLASS_ID: u32 = 0x030a5000;
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::{
            control::effect_simi::EffectSimi,
            game::ctn::{media_block_image::MediaBlockImage, read_file_ref},
        },
        read::{BodyChunk, BodyChunks, BodyReader, Error, ReadBody, read_body_chunks},
    };

    impl ReadBody for MediaBlockImage {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaBlockImage {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl MediaBlockImage {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _effect: Arc<EffectSimi> = r.node_ref()?;
            let _image = read_file_ref(r)?;

            Ok(())
        }
    }
}
