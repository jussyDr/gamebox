//! Item variant list.

/// Item variant list.
#[derive(Default)]
pub struct ItemVariantList;

mod read {
    use std::io::Read;

    use crate::read::{
        reader::{NodeStateMut, Reader},
        Error, ReadBody,
    };

    use super::ItemVariantList;

    impl ReadBody for ItemVariantList {
        fn read_body<R: Read, I, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::version("item variant list", version));
            }

            let _variants = r.list(|r| {
                let _tags = r.list(|r| {
                    r.string()?;
                    r.string()?;

                    Ok(())
                })?;

                let _entity_model = r.external_node_ref::<()>()?;
                let _hidden_in_manual_cycle = r.bool()?;

                Ok(())
            })?;

            Ok(())
        }
    }
}