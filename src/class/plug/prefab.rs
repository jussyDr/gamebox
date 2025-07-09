//! Prefab.

use std::sync::Arc;

use crate::{
    ClassId, ExternalNodeRef, NodeRef, Quat, SubExtensions, Vec3,
    class::plug::{
        dyna_object_model::DynaObjectModel,
        dyna_object_model_instance_params::DynaObjectModelInstanceParams,
        item_placement_placement::ItemPlacementPlacement,
        item_placement_placement_group::ItemPlacementPlacementGroup, path::Path,
        static_object_model::StaticObjectModel,
        static_object_model_instance_params::StaticObjectModelInstanceParams,
        trigger_special::TriggerSpecial,
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
    model: Option<EntityModel>,
    rotation: Quat,
    position: Vec3,
    params: Option<EntityParams>,
}

impl Entity {
    /// Model of the entity.
    pub fn model(&self) -> &Option<EntityModel> {
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
#[derive(Clone)]
pub enum EntityModel {
    /// Dynamic object.
    DynaObject(ExternalNodeRef<DynaObjectModel>),
    /// Path.
    Path(Arc<Path>),
    /// Prefab.
    Prefab(ExternalNodeRef<Prefab>),
    /// Static object model.
    StaticObject(NodeRef<StaticObjectModel>),
    /// Trigger special.
    TriggerSpecial(Arc<TriggerSpecial>),
}

/// Prefab entity parameters.
pub enum EntityParams {
    /// Dyna object model instance params.
    DynaObject(DynaObjectModelInstanceParams),
    /// Item placement.
    ItemPlacement(ItemPlacementPlacement),
    /// Placement group.
    ItemPlacementPlacementGroup(ItemPlacementPlacementGroup),
    /// Static object.
    StaticObject(StaticObjectModelInstanceParams),
}

mod read {
    use std::{any::Any, marker::PhantomData, sync::Arc};

    use crate::{
        ClassId, ExternalNodeRef, NodeRef, SubExtensions,
        class::plug::{
            dyna_object_model::DynaObjectModel,
            path::Path,
            prefab::{Entity, EntityModel, EntityParams, Prefab},
            static_object_model::StaticObjectModel,
            trigger_special::TriggerSpecial,
        },
        read::{
            Error, HeaderChunk, HeaderChunks, ReadBody, Readable, error_unknown_version,
            read_node_from_body,
            reader::{BodyReader, HeaderReader, ReadNodeRef},
        },
        sub_extension,
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
                let model = r.node_ref()?;
                let rotation = r.quat()?;
                let position = r.vec3()?;
                let params = r.node_or_null_generic(|r, class_id| match class_id {
                    0x2f0a9000 => Ok(EntityParams::ItemPlacement(read_node_from_body(r)?)),
                    0x2f0b6000 => Ok(EntityParams::DynaObject(read_node_from_body(r)?)),
                    0x2f0d8000 => Ok(EntityParams::ItemPlacementPlacementGroup(
                        read_node_from_body(r)?,
                    )),
                    0x2f0d9000 => Ok(EntityParams::StaticObject(read_node_from_body(r)?)),
                    _ => todo!("0x{class_id:08x?}"),
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

    impl ReadNodeRef for Option<EntityModel> {
        fn from_node_ref_any(node_ref: NodeRef<dyn Any + Send + Sync>) -> Result<Self, Error> {
            match node_ref {
                NodeRef::Internal(node_ref) => {
                    let node_ref = node_ref
                        .downcast()
                        .map(EntityModel::Path)
                        .or_else(|value| {
                            value.downcast().map(|node_ref| {
                                EntityModel::StaticObject(NodeRef::Internal(node_ref))
                            })
                        })
                        .or_else(|value| value.downcast().map(EntityModel::TriggerSpecial))
                        .map_err(|_| Error::new(""))?;

                    Ok(Some(node_ref))
                }
                NodeRef::External(node_ref) => {
                    let sub_extension = sub_extension(&node_ref.path).unwrap();

                    if DynaObjectModel::has_sub_extension(sub_extension) {
                        Ok(Some(EntityModel::DynaObject(ExternalNodeRef {
                            path: node_ref.path,
                            ancestor_level: node_ref.ancestor_level,
                            marker: PhantomData,
                        })))
                    } else if Prefab::has_sub_extension(sub_extension) {
                        Ok(Some(EntityModel::Prefab(ExternalNodeRef {
                            path: node_ref.path,
                            ancestor_level: node_ref.ancestor_level,
                            marker: PhantomData,
                        })))
                    } else if StaticObjectModel::has_sub_extension(sub_extension) {
                        Ok(Some(EntityModel::StaticObject(NodeRef::External(
                            ExternalNodeRef {
                                path: node_ref.path,
                                ancestor_level: node_ref.ancestor_level,
                                marker: PhantomData,
                            },
                        ))))
                    } else {
                        todo!("{node_ref:?}")
                    }
                }
            }
        }

        fn read_node_ref_internal(
            r: &mut impl BodyReader,
            class_id: u32,
        ) -> Result<Arc<dyn Any + Send + Sync>, Error> {
            match class_id {
                Path::CLASS_ID => Ok(Arc::new(read_node_from_body::<Path>(r)?)),
                StaticObjectModel::CLASS_ID => {
                    Ok(Arc::new(read_node_from_body::<StaticObjectModel>(r)?))
                }
                TriggerSpecial::CLASS_ID => Ok(Arc::new(read_node_from_body::<TriggerSpecial>(r)?)),
                _ => todo!("0x{class_id:08x?}"),
            }
        }

        fn none() -> Result<Self, Error> {
            Ok(None)
        }
    }
}
