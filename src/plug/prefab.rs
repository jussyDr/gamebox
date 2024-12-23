//! Prefab.

use crate::{Class, Quat, Vec3};

use super::{
    dyna_kinematic_contraint::DynaKinematicConstraint, DynaObjectModel, StaticObjectModel,
};

/// Prefab.
#[derive(Default, Debug)]
pub struct Prefab {
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
    ty: PrefabEntityType,
    rotation: Quat,
    position: Vec3<f32>,
}

impl PrefabEntity {
    /// Type.
    pub const fn ty(&self) -> &PrefabEntityType {
        &self.ty
    }

    /// Rotation.
    pub const fn rotation(&self) -> Quat {
        self.rotation
    }

    /// Position.
    pub const fn position(&self) -> Vec3<f32> {
        self.position
    }
}

/// Prefab entity type.
#[derive(Debug)]
pub enum PrefabEntityType {
    /// Dynamic object model.
    DynaObjectModel(DynaObjectModel),
    /// Static object model.
    StaticObjectModel(StaticObjectModel),
    /// Dynamic kinematic constraint.
    DynaKinematicConstraint(DynaKinematicConstraint),
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::{
            dyna_kinematic_contraint::DynaKinematicConstraint,
            static_object_model::StaticObjectModel, DynaObjectModel, Path,
        },
        read::{
            readable::{HeaderChunk, HeaderChunks, Sealed},
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, ErrorKind, ReadBody, Readable,
        },
    };

    use super::{Prefab, PrefabEntity, PrefabEntityType};

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
                let mut ty = PrefabEntityType::StaticObjectModel(StaticObjectModel::default());

                r.test_or_ext_or_null(|r, class_id| {
                    match class_id {
                        0x09119000 => {
                            let mut m = Path::default();
                            m.read_body(r)?;
                        }
                        0x09144000 => {
                            let mut m = DynaObjectModel::default();
                            m.read_body(r)?;

                            ty = PrefabEntityType::DynaObjectModel(m);
                        }
                        0x09159000 => {
                            let mut m = StaticObjectModel::default();
                            m.read_body(r)?;

                            ty = PrefabEntityType::StaticObjectModel(m);
                        }
                        0x09179000 => {
                            // NPlugTrigger_SSpecial

                            r.u32()?;
                            r.u32()?;
                            r.u32()?;
                        }
                        0x2f0ca000 => {
                            let mut m = DynaKinematicConstraint;
                            m.read_body(r)?;

                            ty = PrefabEntityType::DynaKinematicConstraint(m);
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
                        let _position_1 = r.vec3::<f32>()?;
                        let _position_2 = r.vec3::<f32>()?;
                    }
                    0x2f0d8000 => {
                        let version = r.u32()?;

                        if version != 1 {
                            return Err(Error::version("instance params", version));
                        }

                        let _placements = r.list(|r| read_item_placement_placement(r))?;
                        r.list(|r| r.u16())?;
                        r.list(|r| {
                            r.vec3::<f32>()?;
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
