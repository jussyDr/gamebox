//! Item placement placement group.

/// Item placement placement group.
#[derive(Default)]
pub struct ItemPlacementPlacementGroup;

mod read {
    use crate::{
        class::plug::{
            item_placement_placement::ItemPlacementPlacement,
            item_placement_placement_group::ItemPlacementPlacementGroup,
        },
        read::{BodyReader, Error, ReadBody, error_unknown_chunk_version, read_node_from_body},
    };

    impl ReadBody for ItemPlacementPlacementGroup {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            let _placements = r.list(|r| read_node_from_body::<ItemPlacementPlacement>(r))?;
            r.list(|r| r.u16())?;
            r.list(|r| {
                r.vec3()?;
                r.quat()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
