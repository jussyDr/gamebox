//! Media block triangles.

use bytemuck::cast_slice;
use ordered_float::OrderedFloat;

use crate::{Class, OrderedVec3, Vec3};

/// A media block triangles.
#[derive(PartialEq, Eq, Hash, Default, Debug)]
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
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Key {
    time: OrderedFloat<f32>,
    positions: Vec<OrderedVec3>,
}

impl Key {
    /// Time.
    pub const fn time(&self) -> f32 {
        self.time.0
    }

    /// Positions.
    pub fn positions(&self) -> &[Vec3] {
        cast_slice(&self.positions)
    }
}

mod read {
    use std::io::Read;

    use ordered_float::OrderedFloat;

    use crate::read::{reader::Reader, BodyChunk, BodyChunks, Error, ErrorKind};

    use super::{Key, MediaBlockTriangles};

    impl BodyChunks for MediaBlockTriangles {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(1, Self::read_chunk_1),
                BodyChunk::skippable(2, |s, r| Self::read_chunk_2(s, r)),
            ]
            .into_iter()
        }
    }

    impl MediaBlockTriangles {
        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.keys = r.list(|r| {
                let time = r.f32()?;

                Ok(Key {
                    time: OrderedFloat(time),
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

                key.positions = r.repeat(num_verts as usize, |r| r.vec3_ordered())?;
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

mod write {
    use std::io::Write;

    use crate::write::{
        writable::{write_body_chunks, WriteBody},
        writer::{IdStateMut, NodeStateMut},
        BodyChunk, BodyChunks, Error, Writer,
    };

    use super::MediaBlockTriangles;

    impl WriteBody for MediaBlockTriangles {
        fn write_body<W: Write, I: IdStateMut, N: NodeStateMut>(
            &self,
            w: &mut Writer<W, I, N>,
        ) -> Result<(), Error> {
            write_body_chunks(w, self)
        }
    }

    impl BodyChunks for MediaBlockTriangles {
        fn body_chunks<W: Write, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, W, I, N>> {
            [
                BodyChunk::normal(1, Self::write_chunk_1),
                BodyChunk::skippable(2, |s, w| Self::write_chunk_2(s, w)),
            ]
            .into_iter()
        }
    }

    impl MediaBlockTriangles {
        fn write_chunk_1<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            todo!();

            Ok(())
        }

        fn write_chunk_2<I, N>(&self, w: &mut Writer<impl Write, I, N>) -> Result<(), Error> {
            w.u32(0xffffffff)?;

            Ok(())
        }
    }
}
