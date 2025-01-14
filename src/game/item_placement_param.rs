//! Item placement param.

use crate::{plug::item_placement::ItemPlacement, Class, NodeRef, Quat, Vec3};

/// An item placement param.
#[derive(Default)]
pub struct ItemPlacementParam {
    pivot_positions: Vec<Vec3>,
    pivot_rotations: Vec<Quat>,
    placement: NodeRef<ItemPlacement>,
}

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
        Quat, Vec3,
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
            #![allow(clippy::redundant_closure)]
            [
                BodyChunk::skippable(0, |s, r| Self::read_chunk_0(s, r)),
                BodyChunk::skippable(1, |s, r| Self::read_chunk_1(s, r)),
                BodyChunk::skippable(3, |s, r| Self::read_chunk_3(s, r)),
                BodyChunk::skippable(4, |s, r| Self::read_chunk_4(s, r)),
                BodyChunk::skippable(5, |s, r| Self::read_chunk_5(s, r)),
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

        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.pivot_positions = r.list_pod::<Vec3>()?;
            self.pivot_rotations = r.list_pod::<Quat>()?;

            Ok(())
        }

        fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            let placement_version = r.u32()?;

            if !matches!(placement_version, 6 | 8 | 10) {
                return Err(Error::version("item placement", placement_version));
            }

            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            if placement_version >= 8 {
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            r.list(|r| {
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_5(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.placement = r.node_ref::<ItemPlacement>()?;

            Ok(())
        }
    }
}
