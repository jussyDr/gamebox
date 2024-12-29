//! Media block triangles.

use crate::{Class, Vec3};

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
pub struct Key {
    time: f32,
    positions: Vec<Vec3>,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time
    }

    /// Positions.
    pub const fn positions(&self) -> &Vec<Vec3> {
        &self.positions
    }
}

mod read {
    use std::io::Read;

    use crate::read::{reader::Reader, BodyChunk, BodyChunks, Error, ErrorKind};

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
                let time = r.f32()?;

                Ok(Key {
                    time,
                    positions: Vec::default(),
                })
            })?;

            let num_keys = r.u32()?;
            let num_verts = r.u32()?;

            for key_index in 0..num_keys {
                let key = self
                    .keys
                    .get_mut(key_index as usize)
                    .ok_or_else(|| Error::new(ErrorKind::Format("index".into())))?;

                key.positions = r.repeat(num_verts as usize, |r| r.vec3())?;
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
