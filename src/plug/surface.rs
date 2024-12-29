//! Surface.

use crate::{Class, ExternalNodeRef, Nat3, Vec3};

use super::Material;

/// A surface.
#[derive(Default, Debug)]
pub struct Surface {
    ty: SurfaceType,
    materials: Vec<Option<ExternalNodeRef<Material>>>,
}

impl Class for Surface {
    const CLASS_ID: u32 = 0x0900c000;
}

impl Surface {
    /// Type.
    pub const fn ty(&self) -> &SurfaceType {
        &self.ty
    }

    /// Materials.
    pub const fn materials(&self) -> &Vec<Option<ExternalNodeRef<Material>>> {
        &self.materials
    }
}

/// Surface type.
#[derive(Debug)]
pub enum SurfaceType {
    /// Mesh.
    Mesh {
        /// Vertices.
        vertices: Vec<Vec3>,
        /// Triangles.
        triangles: Vec<Nat3>,
    },
}

impl Default for SurfaceType {
    fn default() -> Self {
        Self::Mesh {
            vertices: Default::default(),
            triangles: Default::default(),
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

                    let vertices = r.list(|r| r.vec3())?;
                    let triangles = r.list(|r| {
                        let triangle = r.nat3()?;
                        r.u32()?;

                        Ok(triangle)
                    })?;

                    SurfaceType::Mesh {
                        vertices,
                        triangles,
                    }
                }
                _ => {
                    return Err(Error::new(ErrorKind::Unsupported(
                        "surface type".to_string(),
                    )))
                }
            };

            r.vec3()?;
            self.materials = r.list(|r| {
                if !r.bool()? {
                    return Err(Error::new(ErrorKind::Unsupported("".into())));
                }

                let material = r.external_node_ref_or_null::<Material>()?;

                Ok(material)
            })?;

            let x = if !self.materials.is_empty() {
                let x = r.list(|r| r.u16())?;

                x.is_empty()
            } else {
                true
            };

            if x {
                r.list(|r| r.u16())?;
            }

            r.u32()?;

            Ok(())
        }
    }
}
