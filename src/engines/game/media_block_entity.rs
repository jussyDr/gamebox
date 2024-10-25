use std::io::Read;

use crate::{
    engines::plug::EntRecordData,
    read::{
        readable::{BodyChunk, BodyChunks},
        Error, IdStateMut, NodeStateMut, Reader,
    },
};

/// An entity media block.
pub struct MediaBlockEntity;

impl BodyChunks for MediaBlockEntity {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(0, |n, r| Self::read_chunk_0(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaBlockEntity {
    fn read_chunk_0(
        &mut self,
        r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
    ) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 6 {
            return Err(Error);
        }

        let _record_data = r.node::<EntRecordData>()?;
        let _start_offset = r.f32()?;
        let _notice_records = r.list(|r| r.u32())?;
        let _no_damage = r.bool()?;
        r.bool()?;
        let _force_light = r.bool()?;
        let _force_hue = r.bool()?;
        let _player_model = r.ident()?;
        r.vec3::<f32>()?;
        let _skin_names = r.list(|r| r.pack_desc())?;

        if r.bool()? {
            todo!()
        }

        let _keys = r.list(|r| {
            let _time = r.f32()?;
            let _lights = r.list(|r| r.u32())?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            let _trail_intensity = r.f32()?;

            Ok(())
        })?;

        Ok(())
    }
}
