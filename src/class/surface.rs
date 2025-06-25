use crate::{Class, ExternalNodeRef};

/// A surface.
#[derive(Default)]
pub struct Surface {
    kind: SurfaceKind,
    materials: Vec<Material>,
}

impl Surface {
    pub fn kind(&self) -> &SurfaceKind {
        &self.kind
    }

    pub fn materials(&self) -> &Vec<Material> {
        &self.materials
    }
}

impl Class for Surface {
    const CLASS_ID: u32 = 0x0900c000;
}

#[derive(Default)]
pub enum SurfaceKind {
    Box,
    #[default]
    Mesh,
}

pub enum Material {
    Internal(InternalMaterial),
    External(ExternalNodeRef),
}

pub enum InternalMaterial {
    Metal,
}

mod read {
    use std::io::Read;

    use crate::{
        class::surface::{InternalMaterial, Material, Surface, SurfaceKind},
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, Readable, read_body_chunks,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl Readable for Surface {}

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
            [BodyChunk::new(0x0900c003, Self::read_chunk_3)]
        }
    }

    impl Surface {
        fn read_chunk_3(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 4 {
                return Err(Error("unknown chunk version".into()));
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
                    Ok(Material::External(r.external_node_ref()?))
                } else {
                    match r.u16()? {
                        4 => Ok(Material::Internal(InternalMaterial::Metal)),
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
