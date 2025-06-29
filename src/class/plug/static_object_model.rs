//! Static object model.

use std::sync::Arc;

use crate::{
    Class, SubExtension, NodeRef,
    class::plug::{solid_2_model::Solid2Model, surface::Surface},
};

/// A static object model.
#[derive(Default)]
pub struct StaticObjectModel {
    mesh: NodeRef<Arc<Solid2Model>>,
    hit_shape: Option<NodeRef<Arc<Surface>>>,
}

impl StaticObjectModel {
    pub fn mesh(&self) -> &NodeRef<Arc<Solid2Model>> {
        &self.mesh
    }

    pub fn hit_shape(&self) -> &Option<NodeRef<Arc<Surface>>> {
        &self.hit_shape
    }
}

impl Class for StaticObjectModel {
    const CLASS_ID: u32 = 0x09159000;
}

impl SubExtension for StaticObjectModel {
    const SUB_EXTENSION: &str = "StaticObject";
}

mod read {
    use std::io::Read;

    use crate::{
        class::plug::static_object_model::StaticObjectModel,
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

            self.mesh = r.node_ref()?;
            self.hit_shape = if r.bool8()? {
                None
            } else {
                Some(r.node_ref()?)
            };

            Ok(())
        }
    }
}
