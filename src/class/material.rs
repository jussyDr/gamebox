use crate::Class;

/// A material.
#[derive(Default)]
pub struct Material;

impl Class for Material {
    const CLASS_ID: u32 = 0x09079000;
}

mod read {
    use std::{io::Read, sync::Arc};

    use crate::{
        class::{material::Material, material_custom::MaterialCustom},
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
                BodyChunk::new(0x09079001, Self::read_chunk_1),
                BodyChunk::new(0x09079007, Self::read_chunk_7),
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
            let custom_material: Arc<MaterialCustom> = r.internal_node_ref()?;

            Ok(())
        }
    }
}
