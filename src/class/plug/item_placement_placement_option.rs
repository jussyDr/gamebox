//! Item placement placement option.

/// Item placement placement option.
#[derive(Default)]
pub struct ItemPlacementPlacementOption;

mod read {
    use crate::{
        class::plug::item_placement_placement_option::ItemPlacementPlacementOption,
        read::{BodyReader, Error, ReadBody},
    };

    impl ReadBody for ItemPlacementPlacementOption {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _required_tags = r.list(|r| {
                let _key = r.string()?;
                let _value = r.string()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
