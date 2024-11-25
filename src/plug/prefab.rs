use crate::{read::reader::NodeRef, Class, Quat, Vec3};

use super::static_object_model::StaticObjectModel;

/// A prefab.
#[derive(Default)]
pub struct Prefab {
    entities: Vec<Entity>,
}

impl Class for Prefab {
    const CLASS_ID: u32 = 0x09145000;
}

impl Prefab {
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
}

/// A prefab entity.
pub struct Entity {
    model: NodeRef<StaticObjectModel>,
    rotation: Quat,
    position: Vec3,
}

impl Entity {
    pub fn model(&self) -> &NodeRef<StaticObjectModel> {
        &self.model
    }

    pub fn rotation(&self) -> [f32; 4] {
        [
            self.rotation.x,
            self.rotation.y,
            self.rotation.z,
            self.rotation.w,
        ]
    }

    pub fn position(&self) -> [f32; 3] {
        [self.position.x, self.position.y, self.position.z]
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::static_object_model::StaticObjectModel,
        read::{
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, ReadBody, Readable, Sealed,
        },
    };

    use super::{Entity, Prefab};

    impl Readable for Prefab {}

    impl Sealed for Prefab {}

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
                let position = r.vec3()?;
                r.u32()?;
                let _u01 = r.string()?;

                Ok(Entity {
                    model,
                    rotation,
                    position,
                })
            })?;

            Ok(())
        }
    }
}
