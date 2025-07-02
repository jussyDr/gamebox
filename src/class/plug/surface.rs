use crate::{ClassId, Extensions, ExternalNodeRef};

/// A surface.
#[derive(Default)]
pub struct Surface {
    kind: SurfaceKind,
    materials: Vec<SurfaceMaterial>,
}

impl Surface {
    pub fn kind(&self) -> &SurfaceKind {
        &self.kind
    }

    pub fn materials(&self) -> &Vec<SurfaceMaterial> {
        &self.materials
    }
}

impl ClassId for Surface {
    const CLASS_ID: u32 = 0x0900c000;
}

impl Extensions for Surface {
    const EXTENSIONS: &[&str] = &["HitShape.Gbx"];
}

#[derive(Default)]
pub enum SurfaceKind {
    Box,
    #[default]
    Mesh,
}

pub enum SurfaceMaterial {
    Internal(InternalMaterial),
    External(ExternalNodeRef),
}

pub enum InternalMaterial {
    Metal,
}

mod read {
    use std::io::Read;

    use crate::{
        class::plug::{
            material::Material,
            surface::{InternalMaterial, Surface, SurfaceKind, SurfaceMaterial},
        },
        read::{
            BodyChunk, BodyChunks, Error, HeaderChunk, HeaderChunks, ReadBody, Readable,
            error_unknown_chunk_version, read_body_chunks,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl Readable for Surface {}

    impl HeaderChunks for Surface {
        fn header_chunks<R, I, N>() -> impl IntoIterator<Item = HeaderChunk<Self, R, I, N>> {
            []
        }
    }

    impl ReadBody for Surface {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Surface {
        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(3, Self::read_chunk_3)]
        }
    }

    impl Surface {
        fn read_chunk_3(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 4 {
                return Err(error_unknown_chunk_version(version));
            }

            let surface_version = if version >= 2 { Some(r.u32()?) } else { None };

            if surface_version != Some(2) {
                panic!()
            }

            self.kind = match r.u32()? {
                6 => {
                    let transform = r.box3d()?;
                    r.u16()?;

                    SurfaceKind::Box
                }
                7 => {
                    match r.u32()? {
                        7 => {
                            let vertices = r.list(|r| r.vec3())?;
                            let triangles = r.list(|r| {
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
                si => todo!("{si}"),
            };
            r.vec3()?;
            self.materials = r.list(|r| {
                if r.bool32()? {
                    Ok(SurfaceMaterial::External(
                        r.external_node_ref::<Material>()?,
                    ))
                } else {
                    match r.u16()? {
                        4 => Ok(SurfaceMaterial::Internal(InternalMaterial::Metal)),
                        xx => todo!("{xx}"),
                    }
                }
            })?;

            if !self.materials.is_empty() {
                r.u32()?;
            }

            r.list(|r| r.u16())?;
            let skel = r.u32()?;

            Ok(())
        }
    }
}
