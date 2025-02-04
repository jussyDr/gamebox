//! Material custom.

use crate::{Class, ExternalNodeRef};

use super::Bitmap;

/// A custom material.
#[derive(Default)]
pub struct MaterialCustom {
    textures: Vec<ExternalNodeRef<Bitmap>>,
}

impl Class for MaterialCustom {
    const CLASS_ID: u32 = 0x0903a000;
}

impl MaterialCustom {
    /// Textures.
    pub const fn textures(&self) -> &Vec<ExternalNodeRef<Bitmap>> {
        &self.textures
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::MaterialCustom;

    impl ReadBody for MaterialCustom {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MaterialCustom {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            #![allow(clippy::redundant_closure)]
            [
                BodyChunk::normal(4, Self::read_chunk_4),
                BodyChunk::normal(10, Self::read_chunk_10),
                BodyChunk::normal(12, Self::read_chunk_12),
                BodyChunk::skippable(15, |s, r| Self::read_chunk_15(s, r)),
                BodyChunk::skippable(17, |s, r| Self::read_chunk_17(s, r)),
                BodyChunk::normal(18, Self::read_chunk_18),
                BodyChunk::normal(19, Self::read_chunk_19),
                BodyChunk::normal(20, Self::read_chunk_20),
                BodyChunk::normal(21, Self::read_chunk_21),
                BodyChunk::normal(22, Self::read_chunk_22),
            ]
            .into_iter()
        }
    }

    impl MaterialCustom {
        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.list(|r| r.u32())?;

            Ok(())
        }

        fn read_chunk_10<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let _gpu_fxs_1 = r.list(|r| {
                r.id()?;
                let count_1 = r.u32()?;
                let count_2 = r.u32()?;
                r.bool()?;

                for _ in 0..count_2 {
                    for _ in 0..count_1 {
                        r.f32()?;
                    }
                }

                Ok(())
            })?;
            let _gpu_fxs_2 = r.list(|r| {
                r.id()?;
                let count_1 = r.u32()?;
                let count_2 = r.u32()?;
                r.bool()?;

                for _ in 0..count_2 {
                    for _ in 0..count_1 {
                        r.f32()?;
                    }
                }

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_12<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let _skip_samplers = r.list(|r| {
                r.id()?;
                r.bool()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_15<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.f32()?;
            r.f32()?;
            r.u32()?;
            r.list(|r| {
                r.id()?;
                r.u32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_17<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_18<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_19(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.u32()?;
            self.textures = vec![];
            r.list(|r| {
                let _texture_id = r.id()?;
                r.u32()?;
                let texture = r.external_node_ref_or_null()?;
                r.u32()?;
                r.u32()?;

                if let Some(texture) = texture {
                    self.textures.push(texture);
                }

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_20<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_21<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_22<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            let x = r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            if x & 3 != 0 {
                r.u32()?;
            }

            Ok(())
        }
    }
}
