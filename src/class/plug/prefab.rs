//! Prefab.

use std::{any::Any, sync::Arc};

use crate::{
    ClassId, NodeRef, Quat, SubExtensions, Vec3,
    class::{
        dyna_object_model_instance_params::DynaObjectModelInstanceParams,
        plug::static_object_model::StaticObjectModel,
    },
};

/// A collection of entities.
#[derive(Default)]
pub struct Prefab {
    file_write_time: u64,
    url: String,
    entities: Vec<Entity>,
}

impl Prefab {
    /// File write time.
    pub fn file_write_time(&self) -> u64 {
        self.file_write_time
    }

    /// URL.
    pub fn url(&self) -> &String {
        &self.url
    }

    /// Entities.
    pub fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}

impl ClassId for Prefab {
    const CLASS_ID: u32 = 0x09145000;
}

impl SubExtensions for Prefab {
    const SUB_EXTENSIONS: &[&str] = &["Prefab"];
}

/// An entity stored in a prefab.
pub struct Entity {
    model: NodeRef<Arc<StaticObjectModel>>,
    rotation: Quat,
    position: Vec3,
    params: Option<EntityParams>,
}

impl Entity {
    /// Model of the entity.
    pub fn model(&self) -> &NodeRef<Arc<StaticObjectModel>> {
        &self.model
    }

    /// Rotation of the entity.
    pub fn rotation(&self) -> &Quat {
        &self.rotation
    }

    /// Position of the entity.
    pub fn position(&self) -> &Vec3 {
        &self.position
    }

    /// Optional additional parameters.
    pub fn params(&self) -> &Option<EntityParams> {
        &self.params
    }
}

/// Prefab entity model.
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

/// Prefab entity parameters.
pub enum EntityParams {
    DynaObjectModelInstanceParams(DynaObjectModelInstanceParams),
}

mod read {
    use std::sync::Arc;

    use crate::{
        class::plug::{
            prefab::{Entity, EntityParams, Prefab},
            static_object_model::StaticObjectModel,
        },
        read::{
            Error, HeaderChunk, HeaderChunks, ReadBody, Readable, error_unknown_version,
            read_node_from_body,
            reader::{BodyReader, HeaderReader},
        },
    };

    impl Readable for Prefab {}

    impl HeaderChunks for Prefab {
        fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>> {
            []
        }
    }

    impl ReadBody for Prefab {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 11 {
                return Err(error_unknown_version("prefab", version));
            }

            self.file_write_time = r.u64()?;
            self.url = r.string()?;
            r.u32()?;
            let num_entities = r.u32()?;
            r.u32()?;
            self.entities = r.repeat(num_entities as usize, |r| {
                let model = r.node_ref_generic(|r, class_id| match class_id {
                    0x09159000 => {
                        let node: StaticObjectModel = read_node_from_body(r)?;
                        Ok(Arc::new(node))
                    }
                    _ => todo!("{class_id:08X?}"),
                })?;
                let rotation = r.quat()?;
                let position = r.vec3()?;
                let params = r.node_or_null_generic(|r, class_id| match class_id {
                    0x2f0b6000 => Ok(EntityParams::DynaObjectModelInstanceParams(
                        read_node_from_body(r)?,
                    )),
                    _ => todo!("{class_id:08X?}"),
                })?;
                r.string()?;

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
