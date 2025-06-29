use std::sync::Arc;

use crate::{ClassId, Extensions, class::plug::material_custom::MaterialCustom};

/// A material.
#[derive(Default)]
pub struct Material {
    custom_material: Arc<MaterialCustom>,
}

impl Material {
    pub fn custom_material(&self) -> &Arc<MaterialCustom> {
        &self.custom_material
    }
}

impl ClassId for Material {
    const CLASS_ID: u32 = 0x09079000;
}

impl Extensions for Material {
    const EXTENSIONS: &[&str] = &["Material.gbx", "Material.Gbx"];
}

mod read {
    use std::io::Read;

    use crate::{
        class::plug::material::Material,
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, Readable, read_body_chunks,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl Readable for Material {}

    impl ReadBody for Material {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Material {
        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
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
        }
    }

    impl Material {
        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_7(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            self.custom_material = r.internal_node_ref()?;

            Ok(())
        }

        fn read_chunk_16<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.f32()?;

            Ok(())
        }

        fn read_chunk_17<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, N>,
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
            r.u32()?;
            r.u32()?;
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
