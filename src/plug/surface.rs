//! Surface.

use crate::{Class, ExternalNodeRef, Vec3};

use super::Material;

/// A surface.
#[derive(Default, Debug)]
pub struct Surface {
    ty: SurfaceType,
    materials: Vec<ExternalNodeRef<Material>>,
}

impl Class for Surface {
    const CLASS_ID: u32 = 0x0900c000;
}

/// Surface type.
#[derive(Debug)]
pub enum SurfaceType {
    /// Mesh surface.
    Mesh {
        /// Vertices.
        vertices: Vec<Vec3<f32>>,
    },
}

impl Default for SurfaceType {
    fn default() -> Self {
        Self::Mesh {
            vertices: Default::default(),
        }
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::material::Material,
        read::{
            read_body_chunks,
            readable::{HeaderChunk, HeaderChunks, Sealed},
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ErrorKind, ReadBody, Readable,
        },
    };

    use super::{Surface, SurfaceType};

    impl Readable for Surface {}

    impl Sealed for Surface {}

    impl HeaderChunks for Surface {
        fn header_chunks<R, I, N>() -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

    impl ReadBody for Surface {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Surface {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(3, Self::read_chunk_3)].into_iter()
        }
    }

    impl Surface {
        fn read_chunk_3(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 4 {
                return Err(Error::chunk_version(version));
            }

            let surface_version = r.u32()?;

            if surface_version != 2 {
                return Err(Error::version("surface", version));
            }

            let surface_type = r.u32()?;

            self.ty = match surface_type {
                7 => {
                    let version = r.u32()?;

                    if version != 7 {
                        return Err(Error::version("surface mesh", version));
                    }

                    let vertices = r.list(|r| r.vec3::<f32>())?;
                    let _triangles = r.list(|r| {
                        r.u32()?;
                        r.u32()?;
                        r.u32()?;
                        r.u32()?;

                        Ok(())
                    })?;

                    SurfaceType::Mesh { vertices }
                }
                _ => {
                    return Err(Error::new(ErrorKind::Unsupported(
                        "surface type".to_string(),
                    )))
                }
            };

            r.vec3::<f32>()?;
            self.materials = r.list(|r| {
                if !r.bool()? {
                    return Err(Error::new(ErrorKind::Unsupported("".into())));
                }

                let material = r.external_node_ref::<Material>()?;

                Ok(material)
            })?;

            if !self.materials.is_empty() {
                r.u32()?;
            }

            r.list(|r| r.u16())?;
            r.u32()?;

            Ok(())
        }
    }
}
