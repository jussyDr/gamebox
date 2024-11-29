//! Material.

use std::sync::Arc;

use crate::Class;

use super::material_custom::MaterialCustom;

/// A material.
#[derive(Default)]
pub struct Material {
    custom_material: Arc<MaterialCustom>,
}

impl Class for Material {
    const CLASS_ID: u32 = 0x09079000;
}

impl Material {
    pub const fn custom_material(&self) -> &Arc<MaterialCustom> {
        &self.custom_material
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::material_custom::MaterialCustom,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody, Readable, Sealed,
        },
    };

    use super::Material;

    impl Readable for Material {}

    impl Sealed for Material {}

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
                BodyChunk::new(1, Self::read_chunk_1),
                BodyChunk::new(7, Self::read_chunk_7),
                BodyChunk::new(16, Self::read_chunk_16),
                BodyChunk::new(17, Self::read_chunk_17),
                BodyChunk::skippable(18, Self::read_chunk_18),
                BodyChunk::skippable(19, Self::read_chunk_19),
                BodyChunk::new(21, Self::read_chunk_21),
                BodyChunk::new(22, Self::read_chunk_22),
                BodyChunk::new(23, Self::read_chunk_23),
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
            self.custom_material = r.internal_node_ref::<MaterialCustom>()?;

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
            r.u32()?;
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

            Ok(())
        }

        fn read_chunk_19<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_21<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.list(|r| r.u32())?;
            r.u32()?;

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
            r.u32()?;

            Ok(())
        }

        fn read_chunk_25<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
