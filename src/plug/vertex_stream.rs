//! Vertex stream.

use crate::{Class, Rgba, Texcoord, Vec3};

/// A vertex stream.
#[derive(Default, Debug)]
pub struct VertexStream {
    normals: Vec<[f32; 3]>,
    positions: Vec<Vec3<f32>>,
    tangents_u: Vec<[f32; 3]>,
    tangents_v: Vec<[f32; 3]>,
    texcoords_0: Vec<Texcoord>,
    colors_0: Option<Vec<Rgba<u8>>>,
    texcoords_1: Option<Vec<Texcoord>>,
    texcoords_2: Option<Vec<Texcoord>>,
}

impl Class for VertexStream {
    const CLASS_ID: u32 = 0x09056000;
}

impl VertexStream {
    /// Normal data.
    pub const fn normals(&self) -> &Vec<[f32; 3]> {
        &self.normals
    }

    /// Position data.
    pub const fn positions(&self) -> &Vec<Vec3<f32>> {
        &self.positions
    }

    /// Tangents U.
    pub const fn tangents_u(&self) -> &Vec<[f32; 3]> {
        &self.tangents_u
    }

    /// Tangents V.
    pub const fn tangents_v(&self) -> &Vec<[f32; 3]> {
        &self.tangents_v
    }

    /// Texcoord data.
    pub const fn texcoords_0(&self) -> &Vec<Texcoord> {
        &self.texcoords_0
    }

    /// Color data.
    pub const fn colors_0(&self) -> Option<&Vec<Rgba<u8>>> {
        self.colors_0.as_ref()
    }

    /// Texcoords 1.
    pub const fn texcoords_1(&self) -> Option<&Vec<Texcoord>> {
        self.texcoords_1.as_ref()
    }

    /// Texcoords 2.
    pub const fn texcoords_2(&self) -> Option<&Vec<Texcoord>> {
        self.texcoords_2.as_ref()
    }
}

struct VertexAttrDesc {
    flags: u32,
}

mod read {
    use std::{
        io::{Read, Seek},
        mem::transmute,
    };

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ErrorKind, ReadBody,
    };

    use super::{VertexAttrDesc, VertexStream};

    impl ReadBody for VertexStream {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for VertexStream {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl VertexStream {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let count = r.u32()?;
            let _flags = r.u32()?;
            r.u32()?;
            let vertex_attr_descs = r.list(|r| {
                let flags = r.u32()?;
                let flags_2 = r.u32()?;

                if flags_2 & 0x00000ffc == 0 {
                    if version == 0 {
                        todo!()
                    }
                } else {
                    r.u16()?;
                    let _offset = r.u16()?;
                }

                Ok(VertexAttrDesc { flags })
            })?;
            r.bool()?;

            for vertex_attr_desc in vertex_attr_descs {
                let ty = (vertex_attr_desc.flags >> 9) & 0x000001ff;
                let weight_count = vertex_attr_desc.flags & 0x000001ff;

                match ty {
                    1 => {
                        let data = r.repeat_f32x2(count as usize)?;

                        match weight_count {
                            10 => {
                                self.texcoords_0 = unsafe { transmute(data) };
                            }
                            11 => {
                                self.texcoords_1 = Some(unsafe { transmute(data) });
                            }
                            12 => {
                                self.texcoords_2 = Some(unsafe { transmute(data) });
                            }
                            _ => todo!("{weight_count}"),
                        }
                    }
                    2 => {
                        let data = r.repeat_f32x3(count as usize)?;

                        match weight_count {
                            0 => self.positions = unsafe { transmute(data) },
                            _ => todo!("{weight_count}"),
                        }
                    }
                    4 => {
                        let data = r.repeat_u8x4(count as usize)?;

                        match weight_count {
                            8 => {
                                self.colors_0 = Some(unsafe { transmute(data) });
                            }
                            _ => todo!("{weight_count}"),
                        }
                    }
                    5 => {
                        let data = r.repeat(count as usize, |r| r.i32())?;
                    }
                    14 => {
                        let data = r.repeat(count as usize, |r| {
                            let val = r.u32()?;

                            let x = ((val & 0x000003ff) as f32) / (0x1ff as f32);
                            let y = (((val >> 10) & 0x000003ff) as f32) / (0x1ff as f32);
                            let z = (((val >> 20) & 0x000003ff) as f32) / (0x1ff as f32);

                            Ok([x, y, z])
                        })?;

                        match weight_count {
                            5 => self.normals = data,
                            18 => self.tangents_u = data,
                            20 => self.tangents_v = data,
                            _ => todo!("{weight_count}"),
                        }
                    }
                    _ => {
                        return Err(Error::new(ErrorKind::Unsupported(
                            "vertex attribute type".to_string(),
                        )));
                    }
                }
            }

            Ok(())
        }
    }
}
