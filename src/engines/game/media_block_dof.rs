use std::io::Read;

use crate::read::{
    readable::{BodyChunk, BodyChunks},
    Error, Reader,
};

/// A depth-of-field media block.
pub struct MediaBlockDOF;

impl BodyChunks for MediaBlockDOF {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 1] = [(2, |n, r| Self::read_chunk_2(n, r), false)];

        chunks.into_iter()
    }
}

impl MediaBlockDOF {
    fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _keys = r.list(|r| {
            let _time = r.u32()?;
            let _z_focus = r.f32()?;
            let _lens_size = r.f32()?;
            let _target = r.u32()?;
            let _target_position = r.vec3::<f32>()?;

            Ok(())
        })?;

        Ok(())
    }
}
