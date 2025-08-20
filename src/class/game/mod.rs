pub mod ctn;

mod waypoint_special_property;
pub use waypoint_special_property::WaypointSpecialProperty;

use crate::read::{BodyReaderImpl, Error, Reader, Result};

fn read_encapsulation<T, R: Reader>(
    r: &mut R,
    read_fn: impl FnOnce(&mut BodyReaderImpl<&mut R>) -> Result<T>,
) -> Result<T> {
    if r.u32()? != 0 {
        return Err(Error::Internal("unknown encapsulation version".into()));
    }

    let size = r.u32()?;

    let mut r = BodyReaderImpl::new(r, vec![].into_boxed_slice());

    read_fn(&mut r)
}
