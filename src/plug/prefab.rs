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
    rotation: Quat,
    pos: Vec3<f32>,
}

impl Entity {
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
        plug::{
            dyna_kinematic_contraint::DynaKinematicConstraint,
            static_object_model::StaticObjectModel, DynaObjectModel,
        },
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
                let _model = r.test_or_ext(|r, class_id| {
                    match class_id {
                        0x09144000 => {
                            let mut m = DynaObjectModel::default();
                            m.read_body(r)?;
                        }
                        0x09159000 => {
                            let mut m = StaticObjectModel::default();
                            m.read_body(r)?;
                        }
                        0x2f0ca000 => {
                            let mut m = DynaKinematicConstraint::default();
                            m.read_body(r)?;
                        }
                        _ => todo!("{:08X?}", class_id),
                    }

                    Ok(())
                })?;
                let rotation = r.quat()?;
                let pos = r.vec3()?;

                match r.u32()? {
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
                    0xffffffff => {}
                    x => todo!("{x:08X?}"),
                }

                r.string()?;

                Ok(Entity { rotation, pos })
            })?;

            Ok(())
        }
    }
}
