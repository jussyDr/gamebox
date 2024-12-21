//! Dyna kinematic constraint.

/// Dyna kinematic constraint.
#[derive(Default, Debug)]
pub struct DynaKinematicConstraint;

mod read {
    use std::io::Read;

    use crate::read::{reader::Reader, Error, ReadBody};

    use super::DynaKinematicConstraint;

    impl ReadBody for DynaKinematicConstraint {
        fn read_body<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::version("dyna kine", version));
            }

            let _subversion = r.u32()?;
            let _is_duration = r.bool()?;
            let _sub_funcs = r.list(|r| {
                let _ease = r.u8()?;
                let _revrerse = r.bool8()?;
                let _duration = r.u32()?;

                Ok(())
            })?;
            let _is_duration = r.bool()?;
            let _sub_funcs = r.list(|r| {
                let _ease = r.u8()?;
                let _revrerse = r.bool8()?;
                let _duration = r.u32()?;

                Ok(())
            })?;
            let _shader_tc_type = r.u32()?;
            let _shader_tc_version = r.u32()?;
            let _shader_tc_anim_func = r.u32()?;
            let _trans_axis = r.u8()?;
            let _trans_min = r.f32()?;
            let _trans_max = r.f32()?;
            let _rot_axis = r.u8()?;
            let _angle_min_deg = r.f32()?;
            let _angle_max_deg = r.f32()?;

            Ok(())
        }
    }
}
