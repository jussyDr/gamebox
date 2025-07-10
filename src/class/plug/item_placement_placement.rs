//! Item placement placement.

/// Item placement placement.
#[derive(Default)]
pub struct ItemPlacementPlacement;

mod read {
    use crate::{
        class::plug::{
            item_placement_placement::ItemPlacementPlacement,
            item_placement_placement_option::ItemPlacementPlacementOption,
        },
        read::{BodyReader, Error, ReadBody, error_unknown_version, read_node_from_body},
    };

    impl ReadBody for ItemPlacementPlacement {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_version("item placement placement", version));
            }

            let _layout = r.u32()?;
            let _options = r.list(|r| read_node_from_body::<ItemPlacementPlacementOption>(r))?;

            Ok(())
        }
    }
}
