use std::io::{Read, Seek};

use crate::read::{
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
    Result,
};

use super::{
    IndexBuffer, Indices, VertexStream, Visual, Visual3D, VisualIndexed, VisualIndexedTriangles,
};

impl ReadBody for VisualIndexedTriangles {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for VisualIndexedTriangles {
    fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x09006001,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual::read_chunk_09006001(&mut n.parent.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x09006005,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual::read_chunk_09006005(&mut n.parent.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x09006009,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual::read_chunk_09006009(&mut n.parent.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x0900600b,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual::read_chunk_0900600b(&mut n.parent.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x0900600f,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual::read_chunk_0900600f(&mut n.parent.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x09006010,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual::read_chunk_09006010(&mut n.parent.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x0902c002,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual3D::read_chunk_0902c002(&mut n.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x0902c004,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    Visual3D::read_chunk_0902c004(&mut n.parent.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x0906a001,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    VisualIndexed::read_chunk_0906a001(&mut n.parent, d)
                }),
            },
        ]
        .into_iter()
    }
}

impl Visual {
    fn read_chunk_09006001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_09006005<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09006009<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0900600b<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_0900600f<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 6
        d.u32()?; // 56
        d.u32()?; // 0
        d.u32()?; // 180
        d.u32()?; // 1
        d.inline_node::<VertexStream>()?;
        d.u32()?; // 0
        d.f32()?; // 12.703503
        d.f32()?; // 15.202776
        d.f32()?; // 1.7036213
        d.f32()?; // 14.653503
        d.f32()?; // 17.410307
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }

    fn read_chunk_09006010<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

impl Visual3D {
    fn read_chunk_0902c002<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0xffffffff

        Ok(())
    }

    fn read_chunk_0902c004<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.u32()?; // 0

        Ok(())
    }
}

impl VisualIndexed {
    fn read_chunk_0906a001<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 1

        let mut node = IndexBuffer::default();
        read_body_chunks(&mut node, d)?;

        self.indices = node.indices;

        Ok(())
    }
}

impl BodyChunks for IndexBuffer {
    fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x09057001,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09057001(n, d)),
        }]
        .into_iter()
    }
}

impl IndexBuffer {
    fn read_chunk_09057001<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 2
        let num_indices = d.u32()?;
        let mut current_index = 0;
        self.indices = Indices::U16(d.repeat(num_indices as usize, |d| {
            let offset = d.i16()?;
            current_index = (current_index as i32 + offset as i32) as u16;

            Ok(current_index)
        })?);

        Ok(())
    }
}

impl ReadBody for VertexStream {
    fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl BodyChunks for VertexStream {
    fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
    ) -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [BodyChunkEntry {
            id: 0x09056000,
            read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09056000(n, d)),
        }]
        .into_iter()
    }
}

impl VertexStream {
    fn read_chunk_09056000<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        let vertex_count = d.u32()?; // 180
        d.u32()?; // 3
        d.u32()?; // 0xffffffff
        let vertex_attributes = d.list(|d| {
            let flags = d.u32()?;
            let offset = d.u32()?; // 0 | 0x30 | 0x40

            if offset != 0 {
                d.u32()?;
            }

            Ok(VertexAttribute {
                kind: (flags & 0x1FF) as u16,
                format: ((flags >> 9) & 0x1FF) as u16,
            })
        })?;
        d.u32()?; // 1
        for vertex_attribute in vertex_attributes {
            match vertex_attribute {
                VertexAttribute { kind: 0, format: 2 } => {
                    self.positions = d.repeat(vertex_count as usize, |d| {
                        let x = d.f32()?;
                        let y = d.f32()?;
                        let z = d.f32()?;

                        Ok([x, y, z])
                    })?;
                }
                VertexAttribute {
                    kind: 5,
                    format: 14,
                } => {
                    d.repeat(vertex_count as usize, |d| {
                        d.u32()?;

                        Ok(())
                    })?;
                }
                VertexAttribute { kind: 8, format: 4 } => {
                    d.repeat(vertex_count as usize, |d| {
                        d.u32()?;

                        Ok(())
                    })?;
                }
                VertexAttribute { kind: 9, format: 4 } => {
                    d.repeat(vertex_count as usize, |d| {
                        d.u32()?;

                        Ok(())
                    })?;
                }
                VertexAttribute {
                    kind: 10,
                    format: 1,
                } => {
                    self.texcoords = d.repeat(vertex_count as usize, |d| {
                        let u = d.f32()?;
                        let v = d.f32()?;

                        Ok([u, v])
                    })?;
                }
                VertexAttribute {
                    kind: 11,
                    format: 1,
                } => {
                    d.repeat(vertex_count as usize, |d| {
                        d.f32()?;
                        d.f32()?;

                        Ok(())
                    })?;
                }
                VertexAttribute {
                    kind: 12,
                    format: 1,
                } => {
                    d.repeat(vertex_count as usize, |d| {
                        d.f32()?;
                        d.f32()?;

                        Ok(())
                    })?;
                }
                VertexAttribute {
                    kind: 18,
                    format: 14,
                } => {
                    d.repeat(vertex_count as usize, |d| {
                        d.u32()?;

                        Ok(())
                    })?;
                }
                VertexAttribute {
                    kind: 20,
                    format: 14,
                } => {
                    d.repeat(vertex_count as usize, |d| {
                        d.u32()?;

                        Ok(())
                    })?;
                }
                _ => todo!(),
            }
        }

        Ok(())
    }
}

struct VertexAttribute {
    kind: u16,
    format: u16,
}
