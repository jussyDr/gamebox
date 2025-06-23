//! Static object model.

use crate::{Class, ExternalNodeRef};

/// Static object model.
#[derive(Default)]
pub struct StaticObjectModel {
    mesh: ExternalNodeRef,
    hit_shape: Option<ExternalNodeRef>,
}

impl StaticObjectModel {
    pub fn mesh(&self) -> &ExternalNodeRef {
        &self.mesh
    }

    pub fn hit_shape(&self) -> &Option<ExternalNodeRef> {
        &self.hit_shape
    }
}

impl Class for StaticObjectModel {
    const CLASS_ID: u32 = 0x09159000;
}

mod read {
    use std::io::Read;

    use crate::{
        class::static_object_model::StaticObjectModel,
        read::{
            Error, ReadBody, Readable,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl Readable for StaticObjectModel {}

    impl ReadBody for StaticObjectModel {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error("unknown static object model version".into()));
            }

            self.mesh = r.external_node_ref()?;
            self.hit_shape = if r.bool8()? {
                None
            } else {
                Some(r.external_node_ref()?)
            };

            Ok(())
        }
    }
}
