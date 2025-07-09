//! Item variant list.

use crate::ClassId;

/// Item variant list.
#[derive(Default)]
pub struct ItemVariantList;

impl ClassId for ItemVariantList {
    const CLASS_ID: u32 = 0x2f0bc000;
}

mod read {
    use crate::{
        class::plug::{item_variant::ItemVariant, item_variant_list::ItemVariantList},
        read::{BodyReader, Error, ReadBody, error_unknown_version, read_node_from_body},
    };

    impl ReadBody for ItemVariantList {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_version("item variant list", version));
            }

            let _variants = r.list(|r| read_node_from_body::<ItemVariant>(r))?;

            Ok(())
        }
    }
}
