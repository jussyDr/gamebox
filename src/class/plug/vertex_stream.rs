use crate::{ClassId, Vec2, Vec3};

/// A mesh.
#[derive(Default)]
pub struct VertexStream {
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords_0: Vec<Vec2>,
    texcoords_1: Vec<Vec2>,
    tangents_u: Vec<Vec3>,
    tangents_v: Vec<Vec3>,
}

impl VertexStream {
    /// Position data.
    pub fn positions(&self) -> &Vec<Vec3> {
        &self.positions
    }

    /// Normal data.
    pub fn normals(&self) -> &Vec<Vec3> {
        &self.normals
    }

    /// Texcoord 0 data.
    pub fn texcoords_0(&self) -> &Vec<Vec2> {
        &self.texcoords_0
    }

    /// Texcoord 1 data.
    pub fn texcoords_1(&self) -> &Vec<Vec2> {
        &self.texcoords_1
    }

    /// Tangent U data.
    pub fn tangents_u(&self) -> &Vec<Vec3> {
        &self.tangents_u
    }

    /// Tangent V data.
    pub fn tangents_v(&self) -> &Vec<Vec3> {
        &self.tangents_v
    }
}

impl ClassId for VertexStream {
    const CLASS_ID: u32 = 0x09056000;
}

struct DataDecl {
    target: VertexTarget,
    format: VertexFormat,
}

#[derive(Debug)]
enum VertexTarget {
    Position = 0,
    Normal = 5,
    Texcoord0 = 10,
    Texcoord1 = 11,
    TangentU = 18,
    TangentV = 20,
}

enum VertexFormat {
    Float32x2 = 1,
    Float32x3 = 2,
    Dec3N = 14,
}

mod read {

    use crate::{
        Vec3,
        class::plug::vertex_stream::{DataDecl, VertexFormat, VertexStream, VertexTarget},
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, error_unknown_chunk_version,
            error_unknown_enum_variant, read_body_chunks, reader::BodyReader,
        },
    };

    impl ReadBody for VertexStream {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for VertexStream {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(0, Self::read_chunk_0)]
        }
    }

    impl VertexStream {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
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

                let target = match flags1 & 0x000001ff {
                    0 => VertexTarget::Position,
                    5 => VertexTarget::Normal,
                    10 => VertexTarget::Texcoord0,
                    11 => VertexTarget::Texcoord1,
                    18 => VertexTarget::TangentU,
                    20 => VertexTarget::TangentV,
                    value => return Err(error_unknown_enum_variant("vertex target", value)),
                };

                let format = match (flags1 >> 9) & 0x000001ff {
                    1 => VertexFormat::Float32x2,
                    2 => VertexFormat::Float32x3,
                    14 => VertexFormat::Dec3N,
                    value => return Err(error_unknown_enum_variant("vertex format", value)),
                };

                Ok(DataDecl { target, format })
            })?;
            r.bool32()?;

            for decl in data_decls {
                match decl.format {
                    VertexFormat::Float32x2 => {
                        let data = r.repeat_zerocopy(count as usize)?;

                        match decl.target {
                            VertexTarget::Texcoord0 => self.texcoords_0 = data,
                            VertexTarget::Texcoord1 => self.texcoords_1 = data,
                            _ => todo!("{:?}", decl.target),
                        }
                    }
                    VertexFormat::Float32x3 => {
                        let data = r.repeat_zerocopy(count as usize)?;

                        match decl.target {
                            VertexTarget::Position => self.positions = data,
                            _ => todo!("{:?}", decl.target),
                        }
                    }
                    VertexFormat::Dec3N => {
                        let data_dec3n: Vec<u32> = r.repeat_zerocopy(count as usize)?;
                        let mut data = Vec::with_capacity(data_dec3n.len());

                        for value in data_dec3n {
                            data.push(parse_dec3n(value));
                        }

                        match decl.target {
                            VertexTarget::Normal => {
                                self.normals = data;
                            }
                            VertexTarget::TangentU => {
                                self.tangents_u = data;
                            }
                            VertexTarget::TangentV => {
                                self.tangents_v = data;
                            }
                            _ => todo!("{:?}", decl.target),
                        }
                    }
                }
            }

            Ok(())
        }
    }

    fn parse_dec3n(dec3n: u32) -> Vec3 {
        // 0x000 -> 0x1ff =    0 -> 511
        // 0x200 -> 0x3ff = -512 ->  -1

        let x = ((dec3n << 22) as i32) >> 22;
        let y = ((dec3n << 12) as i32) >> 22;
        let z = ((dec3n << 2) as i32) >> 22;

        Vec3::new((x as f32) / 511.0, (y as f32) / 511.0, (z as f32) / 511.0)
    }

    #[cfg(test)]
    mod tests {
        use crate::Vec3;

        #[test]
        fn parse_dec3n() {
            for (dec3n, expected) in [
                (0x00000000, Vec3::new(0.0, 0.0, 0.0)),
                (0x000001ff, Vec3::new(1.0, 0.0, 0.0)),
                (0x00000200, Vec3::new(-1.0019569, 0.0, 0.0)),
                (0x00000201, Vec3::new(-1.0, 0.0, 0.0)),
                (0x000003ff, Vec3::new(-0.0019569471, 0.0, 0.0)),
            ] {
                let parsed = super::parse_dec3n(dec3n);
                assert_eq!(parsed, expected)
            }
        }
    }
}
