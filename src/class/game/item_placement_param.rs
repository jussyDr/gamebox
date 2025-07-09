//! Item placement param.

use crate::{ClassId, SubExtensions};

/// Item placement param.
#[derive(Default)]
pub struct ItemPlacementParam;

impl ClassId for ItemPlacementParam {
    const CLASS_ID: u32 = 0x2e020000;
}

impl SubExtensions for ItemPlacementParam {
    const SUB_EXTENSIONS: &[&str] = &["PlaceParam"];
}

mod read {
    use crate::{
        NodeRef,
        class::{
            game::item_placement_param::ItemPlacementParam,
            plug::item_placement_class::ItemPlacementClass,
        },
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version, read_body_chunks,
            reader::BodyReader,
        },
    };

    impl ReadBody for ItemPlacementParam {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for ItemPlacementParam {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::skippable(0, Self::read_chunk_0),
                BodyChunk::skippable(1, Self::read_chunk_1),
                BodyChunk::skippable(4, Self::read_chunk_4),
                BodyChunk::skippable(5, Self::read_chunk_5),
            ]
        }
    }

    impl ItemPlacementParam {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_chunk_version(version));
            }

            let _flags = r.u16()?;
            let _cube_center = r.vec3()?;
            let _cube_size = r.f32()?;
            let _grid_snap_h_step = r.f32()?;
            let _grid_snap_v_step = r.f32()?;
            let _grid_snap_h_offset = r.f32()?;
            let _grid_snap_v_offset = r.f32()?;
            let _fly_v_step = r.f32()?;
            let _fly_v_offset = r.f32()?;
            let _pivot_snap_distance = r.f32()?;

            Ok(())
        }

        fn read_chunk_1(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _pivot_positions = r.list(|r| r.vec3())?;
            let _pivot_rotations = r.list(|r| r.quat())?;

            Ok(())
        }

        fn read_chunk_4(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.list(|r| {
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_5(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _placement_class: NodeRef<ItemPlacementClass> = r.node_ref()?;

            Ok(())
        }
    }
}
