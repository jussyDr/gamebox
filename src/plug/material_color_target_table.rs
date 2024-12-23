//! Material color target table.

use crate::Class;

/// Material color target table.
#[derive(Default, Debug)]
pub struct MaterialColorTargetTable;

impl Class for MaterialColorTargetTable {
    const CLASS_ID: u32 = 0x0915e000;
}

mod read {
    use std::io::Read;

    use crate::read::{reader::Reader, Error, ReadBody};

    use super::MaterialColorTargetTable;

    impl ReadBody for MaterialColorTargetTable {
        fn read_body<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::version("material color target table", version));
            }

            r.list(|r| {
                r.list(|r| r.f32())?;
                r.list(|r| r.f32())?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
