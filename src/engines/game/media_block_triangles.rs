use std::io::Read;

use crate::{
    read::{
        readable::{BodyChunk, BodyChunks},
        Reader,
    },
    Error,
};

/// A triangles media block.
#[derive(Default)]
pub struct MediaBlockTriangles;

impl BodyChunks for MediaBlockTriangles {
    type Parent = Self;

    fn parent(&mut self) -> Option<&mut Self> {
        None
    }

    fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
        let chunks: [BodyChunk<Self, R, I, N>; 2] = [
            (1, |n, r| Self::read_chunk_1(n, r), false),
            (2, |n, r| Self::read_chunk_2(n, r), true),
        ];

        chunks.into_iter()
    }
}

impl MediaBlockTriangles {
    fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let _keys = r.list(|r| {
            let _time = r.f32()?;

            Ok(())
        })?;
        let num_keys = r.u32()?;
        let num_verts = r.u32()?;
        let _key_positions = r.repeat(num_keys as usize, |r| {
            let _positions = r.repeat(num_verts as usize, |r| {
                let _position = r.vec3::<f32>()?;

                Ok(())
            })?;

            Ok(())
        })?;
        let _vertices = r.list(|r| r.vec4::<f32>())?;
        let _triangles = r.list(|r| r.vec3::<f32>())?;
        r.u32()?;
        r.u32()?;
        r.u32()?;
        r.f32()?;
        r.u32()?;
        r.u64()?;

        Ok(())
    }

    fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        r.u32()?;

        Ok(())
    }
}
