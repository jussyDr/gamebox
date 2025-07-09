//! Item variant.

use crate::{
    ExternalNodeRef,
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
    use std::{any::Any, marker::PhantomData};

    use crate::{
        ExternalNodeRef, NodeRef, SubExtensions,
        class::plug::{
            item_variant::{ItemVariant, ItemVariantModel},
            prefab::Prefab,
            static_object_model::StaticObjectModel,
            veget_tree_model::VegetTreeModel,
        },
        read::{Error, ReadBody, ReadNodeRef, reader::BodyReader},
        sub_extension,
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
        fn from_node_ref_any(node_ref: NodeRef<dyn Any + Send + Sync>) -> Result<Self, Error> {
            match node_ref {
                NodeRef::External(node_ref) => {
                    let sub_extension = sub_extension(&node_ref.path).unwrap();

                    if Prefab::has_sub_extension(sub_extension) {
                        Ok(Self::Prefab(ExternalNodeRef {
                            path: node_ref.path,
                            ancestor_level: node_ref.ancestor_level,
                            marker: PhantomData,
                        }))
                    } else if StaticObjectModel::has_sub_extension(sub_extension) {
                        Ok(Self::StaticObject(ExternalNodeRef {
                            path: node_ref.path,
                            ancestor_level: node_ref.ancestor_level,
                            marker: PhantomData,
                        }))
                    } else if VegetTreeModel::has_sub_extension(sub_extension) {
                        Ok(Self::VegetTree(ExternalNodeRef {
                            path: node_ref.path,
                            ancestor_level: node_ref.ancestor_level,
                            marker: PhantomData,
                        }))
                    } else {
                        todo!("{}", node_ref.path.display());
                    }
                }
                _ => todo!(),
            }
        }
    }
}
