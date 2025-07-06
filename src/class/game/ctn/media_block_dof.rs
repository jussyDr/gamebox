//! Media block DOF.

use crate::ClassId;

/// Media block DOF.
#[derive(Default)]
pub struct MediaBlockDOF;

impl ClassId for MediaBlockDOF {
    const CLASS_ID: u32 = 0x03126000;
}

mod read {
    use crate::{
        class::game::ctn::media_block_dof::MediaBlockDOF,
        read::{BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks, reader::BodyReader},
    };

    impl ReadBody for MediaBlockDOF {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaBlockDOF {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(2, Self::read_chunk_2)]
        }
    }

    impl MediaBlockDOF {
        fn read_chunk_2(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _keys = r.list(|r| {
                let _time = r.f32()?;
                let _z_focus = r.f32()?;
                let _lens_size = r.f32()?;
                let _target = r.u32()?;
                let _target_position = r.vec3()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
