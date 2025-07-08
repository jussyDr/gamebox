//! Item variant.

/// Item variant.
#[derive(Default)]
pub struct ItemVariant;

mod read {
    use crate::{
        class::plug::{item_variant::ItemVariant, static_object_model::StaticObjectModel},
        read::{Error, ReadBody, reader::BodyReader},
    };

    impl ReadBody for ItemVariant {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _tags = r.list(|r| {
                let _key = r.string()?;
                let _value = r.string()?;

                Ok(())
            })?;

            let _entity_model = r.external_node_ref::<StaticObjectModel>()?;
            let _hidden_in_manual_cycle = r.bool32()?;

            Ok(())
        }
    }
}
