//! Static object model.

use crate::{Class, read::reader::ExternalNodeRef};

/// Static object model.
#[derive(Default)]
pub struct StaticObjectModel {
    mesh: ExternalNodeRef,
    shape: Option<ExternalNodeRef>,
}

impl Class for StaticObjectModel {
    fn class_id(&self) -> u32 {
        0x09159000
    }
}

mod read {
    use std::io::Read;

    use crate::{
        class::static_object_model::StaticObjectModel,
        read::{
            Error, ReadBody,
            reader::{NodesMut, Reader},
        },
    };

    impl ReadBody for StaticObjectModel {
        fn read_body<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodesMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error("unknown static object model version"));
            }

            self.mesh = r.external_node_ref()?;
            self.shape = if r.bool8()? {
                None
            } else {
                Some(r.external_node_ref()?)
            };

            Ok(())
        }
    }
}
