//! Item variant list.

use crate::ExternalNodeRef;

/// Item variant list.
#[derive(Default, Debug)]
pub struct ItemVariantList {
    variants: Vec<ItemVariant>,
}

/// Item variant.
#[derive(Debug)]
pub struct ItemVariant {
    model: ExternalNodeRef<ItemVariantModel>,
}

impl ItemVariant {
    /// Model.
    pub const fn model(&self) -> &ExternalNodeRef<ItemVariantModel> {
        &self.model
    }
}

/// Item variant model.
pub enum ItemVariantModel {
    /// Static object.
    StaticObject,
    /// Prefab.
    Prefab,
}

mod read {
    use std::io::Read;

    use crate::read::{
        reader::{NodeStateMut, Reader},
        Error, ReadBody,
    };

    use super::{ItemVariant, ItemVariantList};

    impl ReadBody for ItemVariantList {
        fn read_body<R: Read, I, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::version("item variant list", version));
            }

            self.variants = r.list(|r| {
                let _tags = r.list(|r| {
                    r.string()?;
                    r.string()?;

                    Ok(())
                })?;

                let model = r.external_node_ref()?;
                let _hidden_in_manual_cycle = r.bool()?;

                Ok(ItemVariant { model })
            })?;

            Ok(())
        }
    }
}
