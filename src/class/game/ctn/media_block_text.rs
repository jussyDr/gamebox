//! Media block text.

use crate::ClassId;

/// Media block text.
#[derive(Default)]
pub struct MediaBlockText;

impl ClassId for MediaBlockText {
    const CLASS_ID: u32 = 0x030a8000;
}

mod read {
    use crate::{
        class::{control::effect_simi::EffectSimi, game::ctn::media_block_text::MediaBlockText},
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for MediaBlockText {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaBlockText {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(1, Self::read_chunk_1),
                BodyChunk::new(2, Self::read_chunk_2),
            ]
        }
    }

    impl MediaBlockText {
        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _text = r.string()?;
            let _effect = r.internal_node_ref::<EffectSimi>()?;

            Ok(())
        }

        fn read_chunk_2(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _color = r.vec3()?;

            Ok(())
        }
    }
}
