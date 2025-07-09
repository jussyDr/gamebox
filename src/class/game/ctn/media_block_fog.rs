//! Media block fog.

use crate::ClassId;

/// Media block fog.
#[derive(Default)]
pub struct MediaBlockFog;

impl ClassId for MediaBlockFog {
    const CLASS_ID: u32 = 0x03199000;
}

mod read {
    use crate::{
        class::game::ctn::media_block_fog::MediaBlockFog,
        read::{
            BodyChunk, BodyChunks, BodyReader, Error, ReadBody, error_unknown_chunk_version,
            read_body_chunks,
        },
    };

    impl ReadBody for MediaBlockFog {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MediaBlockFog {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl MediaBlockFog {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(error_unknown_chunk_version(version));
            }

            let _keys = r.list(|r| {
                let _time = r.f32()?;
                let _intensity = r.f32()?;
                let _sky_intensity = r.f32()?;
                let _distance = r.f32()?;
                let _coefficient = r.f32()?;
                let _color = r.vec3()?;
                let _clouds_opacity = r.f32()?;
                let _clouds_speed = r.f32()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
