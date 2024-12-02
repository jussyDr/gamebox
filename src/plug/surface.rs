//! Surface.

use crate::Class;

/// A surface.
#[derive(Default)]
pub struct Surface;

impl Class for Surface {
    const CLASS_ID: u32 = 0x0900c000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::material::Material,
        read::{
            read_body_chunks,
            readable::Sealed,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ErrorKind, ReadBody, Readable,
        },
    };

    use super::Surface;

    impl Readable for Surface {}

    impl Sealed for Surface {}

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
            [BodyChunk::new(3, Self::read_chunk_3)].into_iter()
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

            match surface_type {
                7 => {
                    let version = r.u32()?;

                    if version != 7 {
                        return Err(Error::version("surface mesh", version));
                    }

                    let _vertices = r.list(|r| r.vec3::<f32>())?;
                    let _triangles = r.list(|r| {
                        r.u32()?;
                        r.u32()?;
                        r.u32()?;
                        r.u32()?;

                        Ok(())
                    })?;
                }
                _ => {
                    return Err(Error::new(ErrorKind::Unsupported(
                        "surface type".to_string(),
                    )))
                }
            }

            r.vec3::<f32>()?;
            let _materials: Vec<()> = r.list(|r| {
                if r.bool()? {
                    r.external_node_ref::<Material>()?;
                } else {
                    todo!()
                }

                Ok(())
            })?;

            r.u32()?;
            r.list(|r| r.u16())?;
            r.u32()?;

            Ok(())
        }
    }
}
