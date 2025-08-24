pub mod ctn;

mod waypoint_special_property;

use std::io::{self, Take};

pub use waypoint_special_property::WaypointSpecialProperty;

use crate::read::{BodyReaderImpl, Error, Reader, Result};

fn read_encapsulation<T, R: Reader>(
    r: &mut R,
    read_fn: impl FnOnce(&mut BodyReaderImpl<Take<&mut R>>) -> Result<T>,
) -> Result<T> {
    if r.u32()? != 0 {
        return Err(Error::Internal("unknown encapsulation version".into()));
    }

    let size = r.u32()?;

    let mut r = BodyReaderImpl::new(io::Read::take(r, size as u64), vec![].into_boxed_slice());

    read_fn(&mut r)
}
