use crate::{ClassId, Extensions};

/// A dynamic object model.
#[derive(Default)]
pub struct DynaObjectModel;

impl ClassId for DynaObjectModel {
    const CLASS_ID: u32 = 0x09144000;
}

impl Extensions for DynaObjectModel {
    const EXTENSIONS: &[&str] = &["DynaObject.Gbx"];
}

mod read {
    use std::io::Read;

    use crate::{
        Delme,
        class::plug::{
            anim_loc_simple::AnimLocSimple, dyna_object_model::DynaObjectModel,
            solid_2_model::Solid2Model, surface::Surface,
        },
        read::{
            Error, HeaderChunk, HeaderChunks, ReadBody, Readable, error_unknown_version,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl Readable for DynaObjectModel {}

    impl HeaderChunks for DynaObjectModel {
        fn header_chunks<R, I, N>() -> impl IntoIterator<Item = HeaderChunk<Self, R, I, N>> {
            []
        }
    }

    impl ReadBody for DynaObjectModel {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 13 {
                return Err(error_unknown_version("dynamic object model", version));
            }

            let is_static = r.bool32()?;
            let dynamize_on_spawn = r.bool32()?;
            let mesh = r.external_node_ref::<Solid2Model>()?;
            let dyna_shape = r.internal_node_ref_or_null::<Surface>()?;
            let static_shape = r.internal_node_ref_or_null::<Surface>()?;
            let break_speed_kmh = r.f32()?;
            let mass = r.f32()?;
            let light_alive_duration_sc_min = r.f32()?;
            let light_alive_duration_sc_max = r.f32()?;
            r.u32()?;
            r.u32()?;
            r.u8()?;
            r.u8()?;
            r.u32()?;
            r.u32()?;
            r.u8()?;
            r.u32()?;
            r.u32()?;
            let loc_anim = r.external_node_ref_or_null::<AnimLocSimple>()?;
            r.u32()?;
            let loc_anim_is_physical = r.bool32()?;
            let water_model = r.external_node_ref_or_null::<Delme>()?;

            Ok(())
        }
    }
}
