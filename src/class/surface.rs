use crate::Class;

#[derive(Default)]
pub struct Surface;

impl Class for Surface {
    fn class_id(&self) -> u32 {
        0x0900c000
    }
}

mod read {
    use std::io::Read;

    use crate::{
        class::surface::Surface,
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, Readable, read_body_chunks,
            reader::{IdsMut, NodesMut, Reader},
        },
    };

    impl Readable for Surface {}

    impl ReadBody for Surface {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdsMut, impl NodesMut>,
        ) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Surface {
        type Parent = Self;

        fn parent(&mut self) -> Option<&mut Self::Parent> {
            None
        }

        fn body_chunks<R: Read, I: IdsMut, N: NodesMut>()
        -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::new(0x0900c003, Self::read_chunk_3)].into_iter()
        }
    }

    impl Surface {
        fn read_chunk_3(
            &mut self,
            r: &mut Reader<impl Read, impl IdsMut, impl NodesMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 4 {
                return Err(Error("unknown chunk version".into()));
            }

            let surface_version = if version >= 2 { Some(r.u32()?) } else { None };

            match r.u32()? {
                7 => match r.u32()? {
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
                },
                si => todo!("{si}"),
            }

            if version >= 2 {
                r.vec3()?;
            }

            let materials = r.list(|r| {
                if r.bool32()? {
                    r.external_node_ref()
                } else {
                    todo!()
                }
            })?;

            if version >= 4 && !materials.is_empty() {
                r.u32()?;
            }

            if version >= 4 {
                r.list(|r| r.u16())?;
            }

            if version >= 1 {
                let skel = r.u32()?;
            }

            Ok(())
        }
    }
}
