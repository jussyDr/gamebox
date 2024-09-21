use std::io::Read;

use crate::{
    engines::game_data::WaypointSpecialProperty,
    read::{
        readable::{BodyChunk, BodyChunksInline},
        Error, IdStateMut, Reader,
    },
};

/// TODO.
#[derive(Default)]
pub struct AnchoredObject;

impl BodyChunksInline for AnchoredObject {
    fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 3] = [
            (2, |n, r| Self::read_chunk_2(n, r), false),
            (4, |n, r| Self::read_chunk_4(n, r), true),
            (5, |n, r| Self::read_chunk_5(n, r), true),
        ];

        chunks.into_iter()
    }
}

impl AnchoredObject {
    fn read_chunk_2<N>(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, N>,
    ) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 8 {
            return Err(Error);
        }

        let _item_model = r.ident()?;
        let _pitch_yaw_roll = r.vec3::<f32>()?;
        let _block_unit_coord = r.vec3::<u8>()?;
        let _anchor_tree_id = r.id()?;
        let _absolute_position_in_map = r.vec3::<f32>()?;
        let _waypoint_special_property = r.node_inline::<WaypointSpecialProperty>()?;
        let flags = r.u16()?;
        let _pivot_position = r.vec3::<f32>()?;
        let _scale = r.f32()?;

        if flags & 0x0004 != 0 {
            let _pack_desc = r.pack_desc()?;
        }

        r.vec3::<f32>()?;
        r.vec3::<f32>()?;

        Ok(())
    }

    fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 0 {
            return Err(Error);
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
