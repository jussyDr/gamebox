//! Prefab.

use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{Class, ExternalNodeRef, NodeRef, Quat, Vec3};

use super::{
    dyna_kinematic_contraint::DynaKinematicConstraint, DynaObjectModel, EditorHelper, Path,
    SpawnModel, StaticObjectModel,
};

/// Prefab.
#[derive(Default, Debug)]
pub struct Prefab {
    file_write_time: u64,
    entities: Vec<PrefabEntity>,
}

impl Class for Prefab {
    const CLASS_ID: u32 = 0x09145000;
}

impl Prefab {
    /// Entities.
    pub const fn entities(&self) -> &Vec<PrefabEntity> {
        &self.entities
    }
}

/// Prefab entity.
#[derive(Debug)]
pub struct PrefabEntity {
    ty: Option<PrefabEntityType>,
    rotation: Quat,
    position: Vec3,
}

impl PrefabEntity {
    /// Rotation.
    pub const fn rotation(&self) -> Quat {
        self.rotation
    }

    /// Position.
    pub const fn position(&self) -> Vec3 {
        self.position
    }
}

/// Prefab entity type.
#[derive(Debug)]
pub enum PrefabEntityType {
    /// Dynamic kinematic constraint.
    DynaKinematicConstraint(Arc<DynaKinematicConstraint>),
    /// Dynamic object model.
    DynaObjectModel(Arc<DynaObjectModel>),
    /// Editor helper.
    EditorHelper(Arc<EditorHelper>),
    /// Path.
    Path(Arc<Path>),
    /// Prefab.
    Prefab(ExternalNodeRef<Prefab>),
    /// Spawn model.
    SpawnModel(Arc<SpawnModel>),
    /// Static object model.
    StaticObjectModel(Arc<StaticObjectModel>),
    /// Trigger special.
    TriggerSpecial(Arc<TriggerSpecial>),
    /// Trigger waypoint.
    TriggerWaypoint(Arc<TriggerWaypoint>),
}

impl TryFrom<NodeRef<dyn Any + Send + Sync>> for PrefabEntityType {
    type Error = ();

    fn try_from(value: NodeRef<dyn Any + Send + Sync>) -> Result<Self, ()> {
        match value {
            NodeRef::Internal(node_ref) => node_ref
                .downcast()
                .map(Self::DynaKinematicConstraint)
                .or_else(|value| value.downcast().map(Self::DynaObjectModel))
                .or_else(|value| value.downcast().map(Self::EditorHelper))
                .or_else(|value| value.downcast().map(Self::Path))
                .or_else(|value| value.downcast().map(Self::SpawnModel))
                .or_else(|value| value.downcast().map(Self::StaticObjectModel))
                .or_else(|value| value.downcast().map(Self::TriggerSpecial))
                .or_else(|value| value.downcast().map(Self::TriggerWaypoint))
                .map_err(|_| ()),
            NodeRef::External(node_ref) => Ok(Self::Prefab(ExternalNodeRef {
                ancestor_level: node_ref.ancestor_level,
                use_file: node_ref.use_file,
                path: node_ref.path,
                phantom: PhantomData,
            })),
        }
    }
}

#[derive(Debug)]
struct TriggerWaypoint;

#[derive(Debug)]
struct TriggerSpecial;

mod read {
    use std::{
        io::{Read, Seek},
        sync::Arc,
    };

    use crate::{
        plug::{
            dyna_kinematic_contraint::DynaKinematicConstraint,
            static_object_model::StaticObjectModel, DynaObjectModel, EditorHelper, Path,
            SpawnModel, Surface,
        },
        read::{
            readable::{HeaderChunk, HeaderChunks, Sealed},
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, ErrorKind, ReadBody, Readable,
        },
    };

    use super::{Prefab, PrefabEntity, TriggerSpecial, TriggerWaypoint};

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

            self.file_write_time = r.u64()?;
            let _url = r.string()?;
            let _u01 = r.u32()?;
            let num_entities = r.u32()?;
            let _u02 = r.u32()?;
            self.entities = r.repeat(num_entities as usize, |r| {
                let ty = r.node_ref_any_or_null(|r, class_id| {
                    match class_id {
                        0x09119000 => {
                            let mut path = Path::default();
                            path.read_body(r)?;

                            Ok(Arc::new(path))
                        }
                        0x09144000 => {
                            let mut dyna_object_model = DynaObjectModel::default();
                            dyna_object_model.read_body(r)?;

                            Ok(Arc::new(dyna_object_model))
                        }
                        0x09159000 => {
                            let mut static_object_model = StaticObjectModel::default();
                            static_object_model.read_body(r)?;

                            Ok(Arc::new(static_object_model))
                        }
                        0x09178000 => {
                            // NPlugTrigger_SWaypoint

                            let version = r.u32()?;

                            if version != 1 {
                                return Err(Error::version("", version));
                            }

                            let _ty = r.u32()?;
                            let _trigger_shape = r.external_node_ref::<Surface>()?;
                            let _no_respawn = r.bool()?;

                            Ok(Arc::new(TriggerWaypoint))
                        }
                        0x09179000 => {
                            // NPlugTrigger_SSpecial

                            r.u32()?;
                            r.u32()?;
                            r.u32()?;

                            Ok(Arc::new(TriggerSpecial))
                        }
                        0x0917a000 => {
                            let mut spawn_model = SpawnModel::default();
                            spawn_model.read_body(r)?;

                            Ok(Arc::new(spawn_model))
                        }
                        0x0917b000 => {
                            let mut editor_helper = EditorHelper::default();
                            editor_helper.read_body(r)?;

                            Ok(Arc::new(editor_helper))
                        }
                        0x2f0ca000 => {
                            let mut dyna_kinematic_constraint = DynaKinematicConstraint;
                            dyna_kinematic_constraint.read_body(r)?;

                            Ok(Arc::new(dyna_kinematic_constraint))
                        }
                        _ => Err(Error::new(ErrorKind::Unsupported(
                            "prefab entity type".into(),
                        ))),
                    }
                })?;
                let rotation = r.quat()?;
                let position = r.vec3()?;
                let class_id = r.u32()?;

                match class_id {
                    0x2f0a9000 => {
                        read_item_placement_placement(r)?;
                    }
                    0x2f0b6000 => {
                        let version = r.u32()?;

                        if version != 2 {
                            return Err(Error::version("instance params", version));
                        }

                        let _period_sc = r.f32()?;
                        let _texture_id = r.u32()?;
                        let _is_kinematic = r.bool()?;
                        let _period_sc_max = r.f32()?;
                        let _phase_01 = r.f32()?;
                        let _phase_01_max = r.f32()?;
                        r.u32()?;
                    }
                    0x2f0c8000 => {
                        let version = r.u32()?;

                        if version != 0 {
                            return Err(Error::version("instance params", version));
                        }

                        let _ent_1 = r.u32()?;
                        let _ent_2 = r.u32()?;
                        let _position_1 = r.vec3()?;
                        let _position_2 = r.vec3()?;
                    }
                    0x2f0d8000 => {
                        let version = r.u32()?;

                        if version != 1 {
                            return Err(Error::version("instance params", version));
                        }

                        let _placements = r.list(|r| read_item_placement_placement(r))?;
                        r.list(|r| r.u16())?;
                        r.list(|r| {
                            r.vec3()?;
                            r.quat()?;

                            Ok(())
                        })?;
                    }
                    0x2f0d9000 => {
                        let version = r.u32()?;

                        if version != 0 {
                            return Err(Error::version("instance params", version));
                        }

                        let _phase_01 = r.f32()?;
                    }
                    0xffffffff => {}
                    _ => {
                        return Err(Error::new(ErrorKind::Unsupported(
                            "prefab entity parameters".into(),
                        )));
                    }
                }

                r.string()?;

                Ok(PrefabEntity {
                    ty,
                    rotation,
                    position,
                })
            })?;

            Ok(())
        }
    }

    fn read_item_placement_placement<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
        let version = r.u32()?;

        if version != 1 {
            return Err(Error::version("instance params", version));
        }

        r.u32()?;
        r.list(|r| {
            r.list(|r| {
                r.string()?;
                r.string()?;

                Ok(())
            })?;

            Ok(())
        })?;

        Ok(())
    }
}
