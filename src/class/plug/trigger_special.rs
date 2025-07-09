//! Trigger special.

use crate::ClassId;

/// Trigger special.
#[derive(Default)]
pub struct TriggerSpecial;

impl ClassId for TriggerSpecial {
    const CLASS_ID: u32 = 0x09179000;
}

mod read {
    use crate::{
        class::plug::trigger_special::TriggerSpecial,
        read::{BodyReader, Error, ReadBody},
    };

    impl ReadBody for TriggerSpecial {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
