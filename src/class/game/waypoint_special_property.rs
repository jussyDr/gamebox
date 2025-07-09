//! Waypoint special property.

use crate::ClassId;

/// Waypoint special property.
#[derive(Default)]
pub struct WaypointSpecialProperty;

impl ClassId for WaypointSpecialProperty {
    const CLASS_ID: u32 = 0x2e009000;
}

mod read {
    use crate::{
        class::game::waypoint_special_property::WaypointSpecialProperty,
        read::{
            BodyChunk, BodyChunks, BodyReader, Error, ReadBody, error_unknown_chunk_version,
            read_body_chunks,
        },
    };

    impl ReadBody for WaypointSpecialProperty {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for WaypointSpecialProperty {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(0, Self::read_chunk_0),
                BodyChunk::skippable(1, Self::read_chunk_1),
            ]
        }
    }

    impl WaypointSpecialProperty {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(error_unknown_chunk_version(version));
            }

            let _tag = r.string()?;
            let _order = r.u32()?;

            Ok(())
        }

        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            if r.bool32()? {
                todo!();
            }

            Ok(())
        }
    }
}
