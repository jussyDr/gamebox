//! Trigger special.

/// Trigger special.
#[derive(Default)]
pub struct TriggerSpecial;

mod read {
    use crate::{
        class::plug::trigger_special::TriggerSpecial,
        read::{Error, ReadBody, reader::BodyReader},
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
