//! Common item entity model.

use std::sync::Arc;

use crate::{
    plug::{StaticObjectModel, Surface},
    Class, Iso4,
};

/// A common item entity model.
#[derive(Default, Debug)]
pub struct CommonItemEntityModel {
    model: Arc<StaticObjectModel>,
    checkpoint: Option<Checkpoint>,
}

impl Class for CommonItemEntityModel {
    const CLASS_ID: u32 = 0x2e027000;
}

impl CommonItemEntityModel {
    /// Model.
    pub const fn model(&self) -> &Arc<StaticObjectModel> {
        &self.model
    }

    /// Checkpoint.
    pub const fn checkpoint(&self) -> Option<&Checkpoint> {
        self.checkpoint.as_ref()
    }
}

/// Checkpoint.
#[derive(Debug)]
pub struct Checkpoint {
    trigger: Arc<Surface>,
    spawn_position: Iso4,
}

impl Checkpoint {
    /// Trigger.
    pub const fn trigger(&self) -> &Arc<Surface> {
        &self.trigger
    }

    /// Spawn position.
    pub const fn spawn_position(&self) -> &Iso4 {
        &self.spawn_position
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::{StaticObjectModel, Surface},
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::{Checkpoint, CommonItemEntityModel};

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
            let trigger = r.internal_node_ref_or_null::<Surface>()?;
            let spawn_position = r.iso4()?;

            if let Some(trigger) = trigger {
                self.checkpoint = Some(Checkpoint {
                    trigger,
                    spawn_position,
                })
            }

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
            r.iso4()?;
            r.u32()?;

            if version >= 5 {
                r.u8()?;
            }

            Ok(())
        }
    }
}
