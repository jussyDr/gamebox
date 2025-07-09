//! Item variant.

use crate::{
    ExternalNodeRef, SubExtensions,
    class::plug::{
        prefab::Prefab, static_object_model::StaticObjectModel, veget_tree_model::VegetTreeModel,
    },
};

/// Item variant.
#[derive(Default)]
pub struct ItemVariant;

enum ItemVariantModel {
    Prefab(ExternalNodeRef<Prefab>),
    StaticObject(ExternalNodeRef<StaticObjectModel>),
    VegetTree(ExternalNodeRef<VegetTreeModel>),
}

mod read {
    use crate::{
        class::plug::item_variant::{ItemVariant, ItemVariantModel},
        read::{
            Error, ReadBody,
            reader::{BodyReader, ClassIdOrSubExtension, ReadNodeRef},
        },
    };

    impl ReadBody for ItemVariant {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let _tags = r.list(|r| {
                let _key = r.string()?;
                let _value = r.string()?;

                Ok(())
            })?;

            let _entity_model: ItemVariantModel = r.node_ref()?;
            let _hidden_in_manual_cycle = r.bool32()?;

            Ok(())
        }
    }

    impl ReadNodeRef for ItemVariantModel {
        fn read_node_ref(
            r: &mut impl BodyReader,
            class_id: Option<ClassIdOrSubExtension>,
        ) -> Result<Self, Error> {
            todo!()
        }
    }
}
