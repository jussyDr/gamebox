//! Media block color grading.

use crate::ClassId;

/// Media block color grading.
#[derive(Default)]
pub struct MediaBlockColorGrading;

impl ClassId for MediaBlockColorGrading {
    const CLASS_ID: u32 = 0x03186000;
}

mod read {
    use crate::{
        class::game::ctn::{media_block_color_grading::MediaBlockColorGrading, read_file_ref},
        read::{BodyChunk, BodyChunks, BodyReader, Error, ReadBody, read_body_chunks},
    };

    impl ReadBody for MediaBlockColorGrading {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaBlockColorGrading {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(0, Self::read_chunk_0),
                BodyChunk::new(1, Self::read_chunk_1),
            ]
        }
    }

    impl MediaBlockColorGrading {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _image = read_file_ref(r)?;

            Ok(())
        }

        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _keys = r.list(|r| {
                let _time = r.f32()?;
                let _intensity = r.f32()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
