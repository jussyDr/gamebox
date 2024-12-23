//! Material.

use std::sync::Arc;

use crate::{Class, NodeRef};

use super::{material_custom::MaterialCustom, MaterialColorTargetTable};

/// Material.
#[derive(Default)]
pub struct Material {
    custom: Option<Arc<MaterialCustom>>,
    color_tables: Vec<NodeRef<MaterialColorTargetTable>>,
}

impl Class for Material {
    const CLASS_ID: u32 = 0x09079000;
}

impl Material {
    /// Custom.
    pub const fn custom(&self) -> Option<&Arc<MaterialCustom>> {
        self.custom.as_ref()
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::MaterialColorTargetTable,
        read::{
            read_body_chunks, readable,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody, Readable,
        },
    };

    use self::readable::{HeaderChunk, HeaderChunks};

    use super::Material;

    impl Readable for Material {}

    impl readable::Sealed for Material {}

    impl HeaderChunks for Material {
        fn header_chunks<R, I, N>() -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

    impl ReadBody for Material {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Material {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(1, Self::read_chunk_1),
                BodyChunk::normal(7, Self::read_chunk_7),
                BodyChunk::normal(16, Self::read_chunk_16),
                BodyChunk::normal(17, Self::read_chunk_17),
                BodyChunk::skippable(18, Self::read_chunk_18),
                BodyChunk::skippable(19, Self::read_chunk_19),
                BodyChunk::normal(21, Self::read_chunk_21),
                BodyChunk::normal(22, Self::read_chunk_22),
                BodyChunk::normal(23, Self::read_chunk_23),
                BodyChunk::skippable(25, Self::read_chunk_25),
            ]
            .into_iter()
        }
    }

    impl Material {
        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_7(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.custom = r.internal_node_ref_or_null()?;

            Ok(())
        }

        fn read_chunk_16<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.f32()?;

            Ok(())
        }

        fn read_chunk_17<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            r.list(|r| r.id())?;

            Ok(())
        }

        fn read_chunk_18<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            if r.u32()? != 0 {
                r.string()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.f32()?;
                r.u32()?;
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_19<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            if r.bool()? {
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_21(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 7 {
                return Err(Error::chunk_version(version));
            }

            let x = r.u32()?;
            self.color_tables = r.list(|r| r.node_ref::<MaterialColorTargetTable>())?;
            r.u32()?;

            if x == 0xffffffff {
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_22<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_23<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.u32()?;
            r.string()?;

            Ok(())
        }

        fn read_chunk_25<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            if r.bool()? {
                r.f32()?;
            }

            Ok(())
        }
    }
}
