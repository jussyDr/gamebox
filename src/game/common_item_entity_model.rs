//! Common item entity model.

use std::sync::Arc;

use crate::{plug::StaticObjectModel, Class};

/// A common item entity model.
#[derive(Default)]
pub struct CommonItemEntityModel {
    model: Arc<StaticObjectModel>,
}

impl Class for CommonItemEntityModel {
    const CLASS_ID: u32 = 0x2e027000;
}

impl CommonItemEntityModel {
    /// Model.
    pub const fn model(&self) -> &Arc<StaticObjectModel> {
        &self.model
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::StaticObjectModel,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::CommonItemEntityModel;

    impl ReadBody for CommonItemEntityModel {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for CommonItemEntityModel {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [BodyChunk::normal(0, Self::read_chunk_0)].into_iter()
        }
    }

    impl CommonItemEntityModel {
        fn read_chunk_0(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 4 | 6) {
                return Err(Error::chunk_version(version));
            }

            self.model = r.internal_node_ref::<StaticObjectModel>()?;
            let _trigger_shape = r.u32()?;
            r.vec3::<f32>()?;
            r.vec3::<f32>()?;
            r.vec3::<f32>()?;
            r.vec3::<f32>()?;
            r.u32()?;
            r.u32()?;

            if version < 5 {
                r.u32()?;
            }

            r.string()?;
            r.string()?;
            r.string()?;
            r.string()?;
            r.string()?;
            r.vec3::<f32>()?;
            r.vec3::<f32>()?;
            r.vec3::<f32>()?;
            r.vec3::<f32>()?;
            r.u32()?;

            if version >= 5 {
                r.u8()?;
            }

            Ok(())
        }
    }
}
