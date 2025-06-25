use crate::Class;

#[derive(Default)]
pub struct DynaObjectModel;

impl Class for DynaObjectModel {
    const CLASS_ID: u32 = 0x09144000;
}

mod read {
    use std::io::Read;

    use crate::{
        class::{dyna_object_model::DynaObjectModel, surface::Surface},
        read::{
            Error, ReadBody, Readable,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl Readable for DynaObjectModel {}

    impl ReadBody for DynaObjectModel {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 13 {
                return Err(Error("unknown dyna object model version".into()));
            }

            let is_static = r.bool32()?;
            let dynamize_on_spawn = r.bool32()?;
            let mesh = r.external_node_ref()?;
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
            let loc_anim = r.external_node_ref_or_null()?;
            r.u32()?;
            let loc_anim_is_physical = r.bool32()?;
            let water_model = r.external_node_ref_or_null()?;

            Ok(())
        }
    }
}
