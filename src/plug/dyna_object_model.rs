//! Dyna object model.

use std::sync::Arc;

use crate::Class;

use super::Solid2Model;

/// Dyna object model
#[derive(Default, Debug)]
pub struct DynaObjectModel {
    model: Arc<Solid2Model>,
}

impl Class for DynaObjectModel {
    const CLASS_ID: u32 = 0x09144000;
}

impl DynaObjectModel {
    /// Model.
    pub const fn model(&self) -> &Arc<Solid2Model> {
        &self.model
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::Surface,
        read::{
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, ReadBody,
        },
    };

    use super::DynaObjectModel;

    impl ReadBody for DynaObjectModel {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 13 {
                return Err(Error::version("dyna", version));
            }

            let _is_static = r.bool()?;
            let _dynamize_on_spawn = r.bool()?;
            self.model = r.internal_node_ref()?;
            let _dyna_shape = r.internal_node_ref::<Surface>()?;
            let _static_shape = r.internal_node_ref::<Surface>()?;
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
            r.u32()?;
            r.u32()?;
            let _loc_anim_is_physical = r.bool()?;
            r.u32()?;

            Ok(())
        }
    }
}
