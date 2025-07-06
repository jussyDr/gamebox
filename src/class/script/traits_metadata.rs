//! Traits metadata.

use crate::ClassId;

/// Traits metadata.
#[derive(Default)]
pub struct TraitsMetadata;

impl ClassId for TraitsMetadata {
    const CLASS_ID: u32 = 0x11002000;
}

mod read {
    use crate::{
        class::script::traits_metadata::TraitsMetadata,
        read::{Error, ReadBody, error_unknown_version, reader::BodyReader},
    };

    impl ReadBody for TraitsMetadata {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(error_unknown_version("traits metadata", version));
            }

            let _num_types = read_packed_u32(r)?;

            Ok(())
        }
    }

    fn read_packed_u32(r: &mut impl BodyReader) -> Result<u32, Error> {
        let x = r.u8()?;
        let y = if x >= 0x80 { r.u16()? } else { 0 };

        Ok((x & 0x7f) as u32 | (y as u32) << 7)
    }
}
