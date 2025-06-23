use crate::{Class, ExternalNodeRef};

#[derive(Default)]
pub struct Surface {
    kind: SurfaceKind,
    materials: Vec<ExternalNodeRef>,
}

impl Class for Surface {
    const CLASS_ID: u32 = 0x0900c000;
}

#[derive(Default)]
pub enum SurfaceKind {
    #[default]
    Mesh,
}

mod read {
    use std::io::Read;

    use crate::{
        class::surface::{Surface, SurfaceKind},
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
        type Parent = Self;

        fn parent(&mut self) -> Option<&mut Self::Parent> {
            None
        }

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
                    r.external_node_ref()
                } else {
                    todo!()
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
