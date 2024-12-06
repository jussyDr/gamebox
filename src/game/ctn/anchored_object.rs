//! Anchored object.

use crate::Class;

/// An anchored object.
#[derive(PartialEq, Default, Debug)]
pub struct AnchoredObject;

impl Class for AnchoredObject {
    const CLASS_ID: u32 = 0x03101000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::WaypointSpecialProperty,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::AnchoredObject;

    impl ReadBody for AnchoredObject {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for AnchoredObject {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(2, Self::read_chunk_2),
                BodyChunk::skippable(4, Self::read_chunk_4),
                BodyChunk::skippable(5, Self::read_chunk_5),
            ]
            .into_iter()
        }
    }

    impl AnchoredObject {
        fn read_chunk_2(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 8 {
                return Err(Error::chunk_version(version));
            }

            let _item_model_id = r.id_or_null()?;
            let _item_model_collection = r.id_or_null()?;
            let _item_model_author = r.id_or_null()?;
            let _pitch_yaw_roll = r.vec3::<f32>()?;
            let _block_unit_coord = r.vec3::<u8>()?;
            let _anchor_tree_id = r.id_or_null()?;
            let _absolute_position = r.vec3::<f32>()?;
            let _waypoint_special_property =
                r.internal_node_ref_or_null::<WaypointSpecialProperty>()?;
            let flags = r.u16()?;
            let _pivot_position = r.vec3::<f32>()?;
            let _scale = r.f32()?;

            if flags & 4 != 0 {
                let _pack_desc = r.pack_desc()?;
            }

            r.vec3::<f32>()?;
            r.vec3::<f32>()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u8()?;

            Ok(())
        }
    }
}
