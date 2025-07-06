//! Anchored object.

use crate::ClassId;

/// Anchored object.
#[derive(Default)]
pub struct AnchoredObject;

impl ClassId for AnchoredObject {
    const CLASS_ID: u32 = 0x03101000;
}

mod read {
    use crate::{
        class::game::ctn::anchored_object::AnchoredObject,
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
    };

    impl ReadBody for AnchoredObject {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for AnchoredObject {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(2, Self::read_chunk_2),
                BodyChunk::skippable(4, Self::read_chunk_4),
                BodyChunk::skippable(5, Self::read_chunk_5),
            ]
        }
    }

    impl AnchoredObject {
        fn read_chunk_2(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 8 {
                return Err(error_unknown_chunk_version(version));
            }

            let _item_model = r.repeat(3, |r| r.id_or_null())?;
            let _yaw_pitch_roll = r.vec3()?;
            let _block_unit_coord = r.repeat(3, |r| r.u8())?;
            let _anchor_tree_id = r.id_or_null()?;
            let _absolute_position_in_map = r.vec3()?;
            let _waypoint_special_property = r.u32()?;
            let flags = r.u16()?;
            let _pivot_position = r.vec3()?;
            let _scale = r.f32()?;

            if flags & 0x0004 != 0 {
                todo!()
            }

            r.vec3()?;
            r.vec3()?;

            Ok(())
        }

        fn read_chunk_4(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_5(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u8()?;

            Ok(())
        }
    }
}
