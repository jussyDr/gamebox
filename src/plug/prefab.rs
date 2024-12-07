//! Prefab.

use crate::{read::reader::NodeRef, Class, Quat, Vec3};

use super::static_object_model::StaticObjectModel;

/// Prefab.
#[derive(Default)]
pub struct Prefab {
    entities: Vec<Entity>,
}

impl Class for Prefab {
    const CLASS_ID: u32 = 0x09145000;
}

impl Prefab {
    /// Entities of the prefab.
    pub const fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}

/// A prefab entity.
pub struct Entity {
    model: NodeRef<StaticObjectModel>,
    rotation: Quat,
    pos: Vec3<f32>,
}

impl Entity {
    /// Model of the entity.
    pub const fn model(&self) -> &NodeRef<StaticObjectModel> {
        &self.model
    }

    /// Rotation of the entity.
    pub const fn rotation(&self) -> Quat {
        self.rotation
    }

    /// Position of the entity.
    pub const fn pos(&self) -> Vec3<f32> {
        self.pos
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::static_object_model::StaticObjectModel,
        read::{
            readable::{HeaderChunk, HeaderChunks, Sealed},
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, ReadBody, Readable,
        },
    };

    use super::{Entity, Prefab};

    impl Readable for Prefab {}

    impl Sealed for Prefab {}

    impl HeaderChunks for Prefab {
        fn header_chunks<R, I, N>() -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

    impl ReadBody for Prefab {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 11 {
                return Err(Error::version("prefab", version));
            }

            let _file_write_time = r.u64()?;
            let _url = r.string()?;
            let _u01 = r.u32()?;
            let num_entities = r.u32()?;
            let _u02 = r.u32()?;
            self.entities = r.repeat(num_entities as usize, |r| {
                let model = r.node_ref::<StaticObjectModel>()?;
                let rotation = r.quat()?;
                let pos = r.vec3()?;
                r.u32()?;
                let _u01 = r.string()?;

                Ok(Entity {
                    model,
                    rotation,
                    pos,
                })
            })?;

            Ok(())
        }
    }
}
