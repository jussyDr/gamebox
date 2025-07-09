//! Media block transition fade.

use crate::ClassId;

/// Media block transition fade.
#[derive(Default)]
pub struct MediaBlockTransitionFade;

impl ClassId for MediaBlockTransitionFade {
    const CLASS_ID: u32 = 0x030ab000;
}

mod read {
    use crate::{
        class::game::ctn::media_block_transition_fade::MediaBlockTransitionFade,
        read::{BodyChunk, BodyChunks, BodyReader, Error, ReadBody, read_body_chunks},
    };

    impl ReadBody for MediaBlockTransitionFade {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaBlockTransitionFade {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl MediaBlockTransitionFade {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _keys = r.list(|r| {
                let _time = r.f32()?;
                let _opacity = r.f32()?;

                Ok(())
            })?;
            let _color = r.vec3()?;
            r.f32()?;

            Ok(())
        }
    }
}
