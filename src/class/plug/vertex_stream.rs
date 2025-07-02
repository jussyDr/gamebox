use crate::{ClassId, Vec2, Vec3};

/// A vertex stream.
#[derive(Default)]
pub struct VertexStream {
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords_0: Vec<Vec2>,
    texcoords_1: Vec<Vec2>,
    tangent_u: Vec<Vec3>,
    tangent_v: Vec<Vec3>,
}

impl VertexStream {
    pub fn positions(&self) -> &Vec<Vec3> {
        &self.positions
    }

    pub fn texcoords_0(&self) -> &Vec<Vec2> {
        &self.texcoords_0
    }

    pub fn texcoords_1(&self) -> &Vec<Vec2> {
        &self.texcoords_1
    }
}

impl ClassId for VertexStream {
    const CLASS_ID: u32 = 0x09056000;
}

struct DataDecl {
    flags1: u32,
    flags2: u32,
}

enum VertexFormat {
    Float32x2 = 1,
    Float32x3 = 2,
    Dec3N = 14,
}

enum VertexTarget {
    Position = 0,
    Normal = 5,
    Texcoord0 = 10,
    Texcoord1 = 11,
    TangentU = 18,
    TangentV = 20,
}

mod read {
    use std::io::Read;

    use crate::{
        Vec3,
        class::plug::vertex_stream::{DataDecl, VertexStream},
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl ReadBody for VertexStream {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for VertexStream {
        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl VertexStream {
        fn read_chunk_0(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error("unknown chunk version".into()));
            }

            let count = r.u32()?;
            let flags = r.u32()?;
            let stream_model = r.node_ref_or_null::<VertexStream>()?;
            let data_decls = r.list(|r| {
                let flags1 = r.u32()?;
                let flags2 = r.u32()?;

                if flags2 & 0x00000ffc == 0 {
                } else {
                    r.u16()?;
                    let offset = r.u16()?;
                }

                Ok(DataDecl { flags1, flags2 })
            })?;
            r.bool32()?;

            for decl in data_decls {
                let format = (decl.flags1 >> 9) & 0x000001ff;
                let target = decl.flags1 & 0x000001ff;

                match format {
                    1 => {
                        let data = r.repeat_zerocopy(count as usize)?;

                        match target {
                            10 => self.texcoords_0 = data,
                            11 => self.texcoords_1 = data,
                            _ => todo!("{target}"),
                        }
                    }
                    2 => {
                        let data = r.repeat_zerocopy(count as usize)?;

                        match target {
                            0 => self.positions = data,
                            _ => todo!("{target}"),
                        }
                    }
                    14 => {
                        let data_dec3n: Vec<u32> = r.repeat_zerocopy(count as usize)?;
                        let mut data = Vec::with_capacity(data_dec3n.len());

                        for value in data_dec3n {
                            let x = value & 0x000003ff;
                            let y = (value >> 10) & 0x000003ff;
                            let z = (value >> 20) & 0x000003ff;

                            let x = if (x & 0x00000200) != 0 {
                                x - 0x00000400
                            } else {
                                x
                            };

                            let y = if (y & 0x00000200) != 0 {
                                y - 0x00000400
                            } else {
                                y
                            };

                            let z = if (z & 0x00000200) != 0 {
                                z - 0x00000400
                            } else {
                                z
                            };

                            data.push(Vec3::new(
                                (x as f32) / 511.0,
                                (y as f32) / 511.0,
                                (z as f32) / 511.0,
                            ));
                        }

                        match target {
                            5 => {
                                self.normals = data;
                            }
                            18 => {
                                self.tangent_u = data;
                            }
                            20 => {
                                self.tangent_v = data;
                            }
                            _ => todo!("{target}"),
                        }
                    }
                    _ => todo!("{format}"),
                }
            }

            Ok(())
        }
    }
}
