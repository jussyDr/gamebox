//! Dyna object model.

use crate::{ClassId, SubExtensions};

/// A dynamic object model.
#[derive(Default)]
pub struct DynaObjectModel;

impl ClassId for DynaObjectModel {
    const CLASS_ID: u32 = 0x09144000;
}

impl SubExtensions for DynaObjectModel {
    const SUB_EXTENSIONS: &[&str] = &["DynaObject"];
}

mod read {
    use std::sync::Arc;

    use crate::{
        Delme, ExternalNodeRef,
        class::plug::{
            anim_loc_simple::AnimLocSimple, dyna_object_model::DynaObjectModel,
            solid_2_model::Solid2Model, surface::Surface,
        },
        read::{
            Error, HeaderChunk, HeaderChunks, ReadBody, Readable, error_unknown_version,
            reader::{BodyReader, HeaderReader},
        },
    };

    impl Readable for DynaObjectModel {}

    impl HeaderChunks for DynaObjectModel {
        fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>> {
            []
        }
    }

    impl ReadBody for DynaObjectModel {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 13 {
                return Err(error_unknown_version("dynamic object model", version));
            }

            let _is_static = r.bool32()?;
            let _dynamize_on_spawn = r.bool32()?;
            let _mesh: ExternalNodeRef<Solid2Model> = r.node_ref()?;
            let _dyna_shape: Option<Arc<Surface>> = r.node_ref()?;
            let _static_shape: Option<Arc<Surface>> = r.node_ref()?;
            let _break_speed_kmh = r.f32()?;
            let _mass = r.f32()?;
            let _light_alive_duration_sc_min = r.f32()?;
            let _light_alive_duration_sc_max = r.f32()?;
            r.u32()?;
            r.u32()?;
            r.u8()?;
            r.u8()?;
            r.u32()?;
            r.u32()?;
            r.u8()?;
            r.u32()?;
            r.u32()?;
            let _loc_anim: Option<ExternalNodeRef<AnimLocSimple>> = r.node_ref()?;
            r.u32()?;
            let _loc_anim_is_physical = r.bool32()?;
            let _water_model: Option<ExternalNodeRef<Delme>> = r.node_ref()?;

            Ok(())
        }
    }
}
