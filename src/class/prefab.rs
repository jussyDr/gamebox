//! Prefab

use std::{any::Any, sync::Arc};

use crate::{
    Class, Quat, Vec3,
    class::{
        dyna_object_model_instance_params::DynaObjectModelInstanceParams,
        static_object_model::StaticObjectModel,
    },
    read::reader::NodeRef,
};

/// Prefab
#[derive(Default)]
pub struct Prefab {
    file_write_time: u64,
    url: String,
    entities: Vec<Entity>,
}

impl Prefab {
    pub fn file_write_time(&self) -> u64 {
        self.file_write_time
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}

impl Class for Prefab {
    fn class_id(&self) -> u32 {
        0x09145000
    }
}

/// Prefab entity.
pub struct Entity {
    model: NodeRef,
    rotation: Quat,
    position: Vec3,
    params: Option<EntityParams>,
}

impl Entity {
    pub fn model(&self) -> &NodeRef {
        &self.model
    }

    pub fn rotation(&self) -> &Quat {
        &self.rotation
    }

    pub fn position(&self) -> &Vec3 {
        &self.position
    }

    pub fn params(&self) -> &Option<EntityParams> {
        &self.params
    }
}

pub enum EntityModel {
    StaticObjectModel(Arc<StaticObjectModel>),
}

impl TryFrom<Arc<dyn Any + Send + Sync>> for EntityModel {
    type Error = ();

    fn try_from(value: Arc<dyn Any + Send + Sync>) -> Result<Self, ()> {
        value
            .downcast()
            .map(Self::StaticObjectModel)
            .map_err(|_| ())
    }
}

pub enum EntityParams {
    DynaObjectModelInstanceParams(DynaObjectModelInstanceParams),
}

mod read {
    use std::{io::Read, sync::Arc};

    use crate::{
        class::{
            dyna_object_model_instance_params::DynaObjectModelInstanceParams,
            prefab::{Entity, EntityParams, Prefab},
            static_object_model::StaticObjectModel,
        },
        read::{
            Error, ReadBody, Readable,
            reader::{NodesMut, Reader},
        },
    };

    impl Readable for Prefab {}

    impl ReadBody for Prefab {
        fn read_body<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodesMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 11 {
                return Err(Error("unknown prefab version"));
            }

            self.file_write_time = r.u64()?;
            self.url = r.string()?;
            let u01 = r.u32()?;
            let num_entities = r.u32()?;
            let u02 = r.u32()?;
            self.entities = r.repeat(num_entities as usize, |r| {
                let model = r.node_ref_generic(|r, class_id| match class_id {
                    0x09159000 => {
                        let mut node = StaticObjectModel::default();
                        node.read_body(r)?;

                        Ok(Arc::new(node))
                    }
                    _ => todo!("{class_id:08X?}"),
                })?;

                let rotation = r.quat()?;
                let position = r.vec3()?;
                let params = r.node_or_null_generic(|r, class_id| match class_id {
                    0x2f0b6000 => {
                        let mut node = DynaObjectModelInstanceParams::default();
                        node.read_body(r)?;

                        Ok(EntityParams::DynaObjectModelInstanceParams(node))
                    }
                    _ => todo!("{class_id:08X?}"),
                })?;
                let _u03 = r.string()?;

                Ok(Entity {
                    model,
                    rotation,
                    position,
                    params,
                })
            })?;

            Ok(())
        }
    }
}
