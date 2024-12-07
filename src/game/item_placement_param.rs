//! Item placement param.

use crate::Class;

/// An item placement param.
#[derive(Default)]
pub struct ItemPlacementParam;

impl Class for ItemPlacementParam {
    const CLASS_ID: u32 = 0x2e020000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::item_placement::ItemPlacement,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::ItemPlacementParam;

    impl ReadBody for ItemPlacementParam {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for ItemPlacementParam {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::skippable(0, Self::read_chunk_0),
                BodyChunk::skippable(1, Self::read_chunk_1),
                BodyChunk::skippable(3, Self::read_chunk_3),
                BodyChunk::skippable(4, Self::read_chunk_4),
                BodyChunk::skippable(5, Self::read_chunk_5),
            ]
            .into_iter()
        }
    }

    impl ItemPlacementParam {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let _flags = r.u16()?;
            let _cube_center = r.vec3::<f32>()?;
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

        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _pivot_positions = r.list(|r| r.vec3::<f32>())?;
            let _pivot_rotations = r.list(|r| r.quat())?;

            Ok(())
        }

        fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            let v = r.u32()?;

            if !matches!(v, 6 | 8) {
                return Err(Error::version("", v));
            }

            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            if v >= 8 {
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_5(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.internal_node_ref::<ItemPlacement>()?;

            Ok(())
        }
    }
}
