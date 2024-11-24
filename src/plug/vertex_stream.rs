use crate::Class;

/// A vertex stream.
#[derive(Default)]
pub struct VertexStream {
    normals: Vec<[f32; 3]>,
    positions: Vec<[f32; 3]>,
    tangent_u: Vec<[f32; 3]>,
    tangent_v: Vec<[f32; 3]>,
    texcoords_0: Vec<[f32; 2]>,
    color_0: Option<Vec<[u8; 4]>>,
    texcoords_1: Option<Vec<[f32; 2]>>,
}

impl Class for VertexStream {
    const CLASS_ID: u32 = 0x09056000;
}

impl VertexStream {
    pub fn normals(&self) -> &[[f32; 3]] {
        &self.normals
    }

    pub fn positions(&self) -> &[[f32; 3]] {
        &self.positions
    }

    pub fn tangent_u(&self) -> &[[f32; 3]] {
        &self.tangent_u
    }

    pub fn tangent_v(&self) -> &[[f32; 3]] {
        &self.tangent_v
    }

    pub fn texcoords_0(&self) -> &[[f32; 2]] {
        &self.texcoords_0
    }

    pub fn color_0(&self) -> Option<&[[u8; 4]]> {
        self.color_0.as_deref()
    }

    pub fn texcoords_1(&self) -> Option<&[[f32; 2]]> {
        self.texcoords_1.as_deref()
    }
}

struct DataDecl {
    flags: u32,
}

mod read {
    use std::io::Read;

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ErrorKind, ReadBody,
    };

    use super::{DataDecl, VertexStream};

    impl ReadBody for VertexStream {
        fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for VertexStream {
        fn body_chunks<R: Read, I, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(0, Self::read_chunk_0)].into_iter()
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
            let data_decls = r.list(|r| {
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

                Ok(DataDecl { flags })
            })?;
            r.bool()?;

            for data_decl in data_decls {
                let ty = (data_decl.flags >> 9) & 0x000001ff;
                let weight_count = data_decl.flags & 0x000001ff;

                match ty {
                    1 => {
                        let data = r.repeat(count as usize, |r| {
                            let u = r.f32()?;
                            let v = r.f32()?;

                            Ok([u, v])
                        })?;

                        match weight_count {
                            10 => self.texcoords_0 = data,
                            11 => self.texcoords_1 = Some(data),
                            _ => todo!("{weight_count}"),
                        }
                    }
                    2 => match weight_count {
                        0 => {
                            self.positions = r.repeat(count as usize, |r| {
                                let x = r.f32()?;
                                let y = r.f32()?;
                                let z = r.f32()?;

                                Ok([x, y, z])
                            })?;
                        }
                        _ => todo!("{weight_count}"),
                    },
                    4 => match weight_count {
                        8 => {
                            self.color_0 = Some(r.repeat(count as usize, |r| {
                                let a = r.u8()?;
                                let b = r.u8()?;
                                let c = r.u8()?;
                                let d = r.u8()?;

                                Ok([a, b, c, d])
                            })?);
                        }
                        _ => todo!("{weight_count}"),
                    },
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
                            18 => self.tangent_u = data,
                            20 => self.tangent_v = data,
                            _ => todo!("{weight_count}"),
                        }
                    }
                    _ => {
                        return Err(Error::new(ErrorKind::Unsupported(
                            "data decl type".to_string(),
                        )))
                    }
                }
            }

            Ok(())
        }
    }
}
