//! Static object model instance params.

use crate::ClassId;

/// Static object model instance params.
#[derive(Default)]
pub struct StaticObjectModelInstanceParams;

impl ClassId for StaticObjectModelInstanceParams {
    const CLASS_ID: u32 = 0x2f0d9000;
}

mod read {
    use crate::{
        class::plug::static_object_model_instance_params::StaticObjectModelInstanceParams,
        read::{BodyReader, Error, ReadBody, error_unknown_version},
    };

    impl ReadBody for StaticObjectModelInstanceParams {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(error_unknown_version(
                    "static object model instance params",
                    version,
                ));
            }

            let _phase01 = r.f32()?;

            Ok(())
        }
    }
}
