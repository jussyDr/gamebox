//! Dyna object model instance params.

/// Dynamic object model instance params.
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

mod read {
    use crate::{
        class::dyna_object_model_instance_params::DynaObjectModelInstanceParams,
        read::{Error, ReadBody, error_unknown_version, reader::BodyReader},
    };

    impl ReadBody for DynaObjectModelInstanceParams {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(error_unknown_version(
                    "dynamic object model instance params",
                    version,
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
