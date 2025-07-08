//! Surface.

use crate::{ClassId, ExternalNodeRef, SubExtensions};

/// A surface.
#[derive(Default)]
pub struct Surface {
    kind: SurfaceKind,
    materials: Vec<SurfaceMaterial>,
}

impl Surface {
    /// Surface kind.
    pub fn kind(&self) -> &SurfaceKind {
        &self.kind
    }

    /// Materials.
    pub fn materials(&self) -> &Vec<SurfaceMaterial> {
        &self.materials
    }
}

impl ClassId for Surface {
    const CLASS_ID: u32 = 0x0900c000;
}

impl SubExtensions for Surface {
    const SUB_EXTENSIONS: &[&str] = &["HitShape"];
}

#[derive(Default)]
pub enum SurfaceKind {
    Box,
    #[default]
    Mesh,
}

pub enum SurfaceMaterial {
    Internal(InternalMaterial),
    External(Option<ExternalNodeRef>),
}

pub enum InternalMaterial {
    Metal,
}

mod read {
    use crate::{
        class::plug::{
            material::Material,
            surface::{InternalMaterial, Surface, SurfaceKind, SurfaceMaterial},
        },
        read::{
            BodyChunk, BodyChunks, Error, HeaderChunk, HeaderChunks, ReadBody, Readable,
            error_unknown_chunk_version, error_unknown_enum_variant, error_unknown_version,
            read_body_chunks,
            reader::{BodyReader, HeaderReader},
        },
    };

    impl Readable for Surface {}

    impl HeaderChunks for Surface {
        fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>> {
            []
        }
    }

    impl ReadBody for Surface {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Surface {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [BodyChunk::new(3, Self::read_chunk_3)]
        }
    }

    impl Surface {
        fn read_chunk_3(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 4 {
                return Err(error_unknown_chunk_version(version));
            }

            let surface_version = r.u32()?;

            if surface_version != 2 {
                return Err(error_unknown_version("surface", surface_version));
            }

            self.kind = match r.u32()? {
                6 => {
                    let _transform = r.box3d()?;
                    r.u16()?;

                    SurfaceKind::Box
                }
                7 => {
                    match r.u32()? {
                        7 => {
                            let _vertices = r.list(|r| r.vec3())?;
                            let _triangles = r.list(|r| {
                                r.u32()?;
                                r.u32()?;
                                r.u32()?;
                                r.u8()?;
                                r.u8()?;
                                r.u8()?;
                                r.u8()?;

                                Ok(())
                            })?;
                        }
                        sv => todo!("{sv}"),
                    }

                    SurfaceKind::Mesh
                }
                value => return Err(error_unknown_enum_variant("surface kind", value)),
            };
            r.vec3()?;
            self.materials = r.list(|r| {
                if r.bool32()? {
                    Ok(SurfaceMaterial::External(
                        r.external_node_ref_or_null::<Material>()?,
                    ))
                } else {
                    match r.u16()? {
                        4 => Ok(SurfaceMaterial::Internal(InternalMaterial::Metal)),
                        xx => todo!("{xx}"),
                    }
                }
            })?;

            if !self
                .materials
                .iter()
                .any(|material| matches!(material, SurfaceMaterial::External(None)))
            {
                r.u32()?;
            }

            r.list(|r| r.u16())?;
            let _skel = r.u32()?;

            Ok(())
        }
    }
}
