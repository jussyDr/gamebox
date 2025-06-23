use crate::{Class, Vec2, Vec3};

#[derive(Default)]
pub struct VertexStream {
    positions: Vec<Vec3>,
    texcoords_0: Vec<Vec2>,
    texcoords_1: Vec<Vec2>,
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

impl Class for VertexStream {
    const CLASS_ID: u32 = 0x09056000;
}

struct DataDecl {
    flags1: u32,
    flags2: u32,
}

mod read {
    use std::io::Read;

    use crate::{
        class::vertex_stream::{DataDecl, VertexStream},
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
        type Parent = Self;

        fn parent(&mut self) -> Option<&mut Self::Parent> {
            None
        }

        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(0x09056000, Self::read_chunk_0)]
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
                match (decl.flags1 >> 9) & 0x000001ff {
                    1 => match decl.flags1 & 0x000001ff {
                        10 => {
                            self.texcoords_0 = r.repeat(count as usize, |r| r.vec2())?;
                        }
                        11 => {
                            self.texcoords_1 = r.repeat(count as usize, |r| r.vec2())?;
                        }
                        wc => todo!("{wc}"),
                    },
                    2 => match decl.flags1 & 0x000001ff {
                        0 => {
                            self.positions = r.repeat(count as usize, |r| r.vec3())?;
                        }
                        wc => todo!("{wc}"),
                    },
                    14 => match decl.flags1 & 0x000001ff {
                        5 => {
                            let normals = r.repeat(count as usize, |r| r.u32())?;
                        }
                        18 => {
                            let tangentu = r.repeat(count as usize, |r| r.u32())?;
                        }
                        20 => {
                            let tangentv = r.repeat(count as usize, |r| r.u32())?;
                        }
                        wc => todo!("{wc}"),
                    },
                    ty => todo!("{ty}"),
                }
            }

            Ok(())
        }
    }
}
