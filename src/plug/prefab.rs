//! Prefab.

use crate::{Class, Quat, Vec3};

use super::{
    dyna_kinematic_contraint::DynaKinematicConstraint, DynaObjectModel, Path, StaticObjectModel,
};

/// Prefab.
#[derive(Default, Debug)]
pub struct Prefab {
    file_write_time: u64,
    entities: Vec<Entity>,
}

impl Class for Prefab {
    const CLASS_ID: u32 = 0x09145000;
}

impl Prefab {
    /// Entities.
    pub const fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}

/// Prefab entity.
#[derive(Debug)]
pub struct Entity {
    ty: EntityType,
    rotation: Quat,
    position: Vec3,
}

impl Entity {
    /// Type.
    pub const fn ty(&self) -> &EntityType {
        &self.ty
    }

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
pub enum EntityType {
    /// Dynamic kinematic constraint.
    DynaKinematicConstraint(DynaKinematicConstraint),
    /// Dynamic object model.
    DynaObjectModel(DynaObjectModel),
    /// Path.
    Path(Path),
    /// Static object model.
    StaticObjectModel(StaticObjectModel),
}

mod read {
    use std::io::{Read, Seek};

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

    use super::{Entity, EntityType, Prefab};

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
                let mut ty = EntityType::StaticObjectModel(StaticObjectModel::default());

                r.test_or_ext_or_null(|r, class_id| {
                    match class_id {
                        0x09119000 => {
                            let mut m = Path::default();
                            m.read_body(r)?;

                            ty = EntityType::Path(m);
                        }
                        0x09144000 => {
                            let mut m = DynaObjectModel::default();
                            m.read_body(r)?;

                            ty = EntityType::DynaObjectModel(m);
                        }
                        0x09159000 => {
                            let mut m = StaticObjectModel::default();
                            m.read_body(r)?;

                            ty = EntityType::StaticObjectModel(m);
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
                        }
                        0x09179000 => {
                            // NPlugTrigger_SSpecial

                            r.u32()?;
                            r.u32()?;
                            r.u32()?;
                        }
                        0x0917a000 => {
                            let mut m = SpawnModel::default();
                            m.read_body(r)?;
                        }
                        0x0917b000 => {
                            let mut m = EditorHelper::default();
                            m.read_body(r)?;
                        }
                        0x2f0ca000 => {
                            let mut m = DynaKinematicConstraint;
                            m.read_body(r)?;

                            ty = EntityType::DynaKinematicConstraint(m);
                        }
                        _ => {
                            return Err(Error::new(ErrorKind::Unsupported(
                                "prefab entity type".into(),
                            )));
                        }
                    }

                    Ok(())
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

                Ok(Entity {
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
