//! Vertex stream.

use crate::{ClassId, Vec2, Vec3};

/// A mesh.
#[derive(Default)]
pub struct VertexStream {
    positions: Vec<Vec3>,
    blend_indices: Option<Vec<u32>>,
    normals: Option<Vec<Vec3>>,
    colors_0: Option<Vec<u32>>,
    texcoords_0: Vec<Vec2>,
    texcoords_1: Option<Vec<Vec2>>,
    texcoords_2: Option<Vec<Vec2>>,
    tangents_u: Option<Vec<Vec3>>,
    tangents_v: Option<Vec<Vec3>>,
}

impl VertexStream {
    /// Position data.
    pub fn positions(&self) -> &Vec<Vec3> {
        &self.positions
    }

    /// Blend index data.
    pub fn blend_indices(&self) -> &Option<Vec<u32>> {
        &self.blend_indices
    }

    /// Normal data.
    pub fn normals(&self) -> &Option<Vec<Vec3>> {
        &self.normals
    }

    /// Color 0 data.
    pub fn colors_0(&self) -> &Option<Vec<u32>> {
        &self.colors_0
    }

    /// Texcoord 0 data.
    pub fn texcoords_0(&self) -> &Vec<Vec2> {
        &self.texcoords_0
    }

    /// Texcoord 1 data.
    pub fn texcoords_1(&self) -> &Option<Vec<Vec2>> {
        &self.texcoords_1
    }

    /// Texcoord 2 data.
    pub fn texcoords_2(&self) -> &Option<Vec<Vec2>> {
        &self.texcoords_2
    }

    /// Tangent U data.
    pub fn tangents_u(&self) -> &Option<Vec<Vec3>> {
        &self.tangents_u
    }

    /// Tangent V data.
    pub fn tangents_v(&self) -> &Option<Vec<Vec3>> {
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
    Position,
    BlendIndices,
    Normal,
    Color0,
    Texcoord0,
    Texcoord1,
    Texcoord2,
    TangentU,
    TangentV,
}

enum VertexFormat {
    Float32x2,
    Float32x3,
    Rgba,
    U32,
    Dec3N,
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
            let _flags = r.u32()?;
            let _stream_model = r.node_ref_or_null::<VertexStream>()?;
            let data_decls = r.list(|r| {
                let flags1 = r.u32()?;
                let flags2 = r.u32()?;

                if flags2 & 0x00000ffc == 0 {
                } else {
                    r.u16()?;
                    let _offset = r.u16()?;
                }

                let target = match flags1 & 0x000001ff {
                    0 => VertexTarget::Position,
                    4 => VertexTarget::BlendIndices,
                    5 => VertexTarget::Normal,
                    8 => VertexTarget::Color0,
                    10 => VertexTarget::Texcoord0,
                    11 => VertexTarget::Texcoord1,
                    12 => VertexTarget::Texcoord2,
                    18 => VertexTarget::TangentU,
                    20 => VertexTarget::TangentV,
                    value => return Err(error_unknown_enum_variant("vertex target", value)),
                };

                let format = match (flags1 >> 9) & 0x000001ff {
                    1 => VertexFormat::Float32x2,
                    2 => VertexFormat::Float32x3,
                    4 => VertexFormat::Rgba,
                    5 => VertexFormat::U32,
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
                            VertexTarget::Texcoord1 => self.texcoords_1 = Some(data),
                            VertexTarget::Texcoord2 => self.texcoords_2 = Some(data),
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
                    VertexFormat::Rgba => {
                        let data: Vec<u32> = r.repeat_zerocopy(count as usize)?;

                        match decl.target {
                            VertexTarget::Color0 => self.colors_0 = Some(data),
                            _ => todo!("{:?}", decl.target),
                        }
                    }
                    VertexFormat::U32 => {
                        let data: Vec<u32> = r.repeat_zerocopy(count as usize)?;

                        match decl.target {
                            VertexTarget::BlendIndices => self.blend_indices = Some(data),
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
                                self.normals = Some(data);
                            }
                            VertexTarget::TangentU => {
                                self.tangents_u = Some(data);
                            }
                            VertexTarget::TangentV => {
                                self.tangents_v = Some(data);
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
