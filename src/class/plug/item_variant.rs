//! Item variant.

use crate::{ExternalNodeRef, SubExtensions};

/// Item variant.
#[derive(Default)]
pub struct ItemVariant;

enum ItemVariantModel {
    Prefab(ExternalNodeRef),
    StaticObject(ExternalNodeRef),
    VegetTree(ExternalNodeRef),
}

impl SubExtensions for ItemVariantModel {
    const SUB_EXTENSIONS: &[&str] = &["Prefab", "StaticObject", "VegetTreeModel"];
}

mod read {
    use crate::{
        class::plug::item_variant::{ItemVariant, ItemVariantModel},
        read::{Error, ReadBody, reader::BodyReader},
    };

    impl ReadBody for ItemVariant {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _tags = r.list(|r| {
                let _key = r.string()?;
                let _value = r.string()?;

                Ok(())
            })?;

            let _entity_model = r.external_node_ref::<ItemVariantModel>()?;
            let _hidden_in_manual_cycle = r.bool32()?;

            Ok(())
        }
    }
}
