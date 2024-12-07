//! Media block triangles.

use crate::Class;

/// A media block triangles.
#[derive(Default)]
pub struct MediaBlockTriangles {
    keys: Vec<Key>,
}

impl Class for MediaBlockTriangles {
    const CLASS_ID: u32 = 0x03029000;
}

impl MediaBlockTriangles {
    /// Keys.
    pub const fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

/// Triangles media block key.
pub struct Key;

mod read {
    use std::io::Read;

    use crate::read::{reader::Reader, BodyChunk, BodyChunks, Error};

    use super::{Key, MediaBlockTriangles};

    impl BodyChunks for MediaBlockTriangles {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(1, Self::read_chunk_1),
                BodyChunk::skippable(2, Self::read_chunk_2),
            ]
            .into_iter()
        }
    }

    impl MediaBlockTriangles {
        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.keys = r.list(|r| {
                let _time = r.f32()?;

                Ok(Key)
            })?;

            let num_keys = r.u32()?;
            let num_verts = r.u32()?;

            for _ in 0..num_keys {
                for _ in 0..num_verts {
                    let _position = r.vec3::<f32>()?;
                }
            }

            let _vertices = r.list(|r| {
                r.f32()?;
                r.f32()?;
                r.f32()?;
                r.f32()?;

                Ok(())
            })?;
            let _triangles = r.list(|r| {
                r.u32()?;
                r.u32()?;
                r.u32()?;

                Ok(())
            })?;
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
}
