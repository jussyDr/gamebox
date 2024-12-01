//! Common item entity model edition.

use std::sync::Arc;

use crate::{plug::crystal::Crystal, Class};

/// A common item entity model edition.
#[derive(Default)]
pub struct CommonItemEntityModelEdition {
    mesh_crystal: Arc<Crystal>,
}

impl Class for CommonItemEntityModelEdition {
    const CLASS_ID: u32 = 0x2e026000;
}

impl CommonItemEntityModelEdition {
    pub const fn mesh_crystal(&self) -> &Arc<Crystal> {
        &self.mesh_crystal
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::crystal::Crystal,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::CommonItemEntityModelEdition;

    impl ReadBody for CommonItemEntityModelEdition {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for CommonItemEntityModelEdition {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::new(0, Self::read_chunk_0),
                BodyChunk::skippable(1, Self::read_chunk_1),
            ]
            .into_iter()
        }
    }

    impl CommonItemEntityModelEdition {
        fn read_chunk_0(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 7 {
                return Err(Error::chunk_version(version));
            }

            let _item_type = r.u32()?;
            self.mesh_crystal = r.internal_node_ref::<Crystal>()?;
            r.string()?;
            r.u32()?;
            r.u32()?;
            let _sprite_params = r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.vec3::<f32>()?;
            r.vec3::<f32>()?;
            r.vec3::<f32>()?;
            r.vec3::<f32>()?;
            r.bool()?;
            r.bool()?;
            r.u32()?;
            let _inventory_name = r.string()?;
            let _inventory_description = r.string()?;
            let _inventory_item_class = r.u32()?;
            let _inventory_occupation = r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
