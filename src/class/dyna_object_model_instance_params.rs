#[derive(Default)]
pub struct DynaObjectModelInstanceParams {
    period_sc: f32,
    texture_id: u32,
    is_kinematic: bool,
    period_sc_max: f32,
    phase01: f32,
    phase01_max: f32,
    cast_static_shadow: bool,
}

impl DynaObjectModelInstanceParams {
    pub fn period_sc(&self) -> f32 {
        self.period_sc
    }

    pub fn texture_id(&self) -> u32 {
        self.texture_id
    }

    pub fn is_kinematic(&self) -> bool {
        self.is_kinematic
    }

    pub fn period_sc_max(&self) -> f32 {
        self.period_sc_max
    }

    pub fn phase01(&self) -> f32 {
        self.phase01
    }

    pub fn phase01_max(&self) -> f32 {
        self.phase01_max
    }

    pub fn cast_static_shadow(&self) -> bool {
        self.cast_static_shadow
    }
}

mod read {
    use std::io::Read;

    use crate::{
        class::dyna_object_model_instance_params::DynaObjectModelInstanceParams,
        read::{
            Error, ReadBody,
            reader::{IdsMut, NodesMut, Reader},
        },
    };

    impl ReadBody for DynaObjectModelInstanceParams {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdsMut, impl NodesMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error(
                    "unknown dyna object model instance params version".into(),
                ));
            }

            self.period_sc = r.f32()?;
            self.texture_id = r.u32()?;
            self.is_kinematic = r.bool32()?;
            self.period_sc_max = r.f32()?;
            self.phase01 = r.f32()?;
            self.phase01_max = r.f32()?;
            self.cast_static_shadow = r.bool32()?;

            Ok(())
        }
    }
}
