//! Editor helper.

use crate::{Class, ExternalNodeRef};

use super::Prefab;

/// Editor helper.
#[derive(Default)]
pub struct EditorHelper {
    prefab: ExternalNodeRef<Prefab>,
}

impl Class for EditorHelper {
    const CLASS_ID: u32 = 0x0917b000;
}

impl EditorHelper {
    /// Prefab.
    pub const fn prefab(&self) -> &ExternalNodeRef<Prefab> {
        &self.prefab
    }
}

mod read {
    use std::io::Read;

    use crate::{
        plug::Prefab,
        read::{
            reader::{NodeStateMut, Reader},
            Error, ReadBody,
        },
    };

    use super::EditorHelper;

    impl ReadBody for EditorHelper {
        fn read_body<R: Read, I, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            self.prefab = r.external_node_ref::<Prefab>()?;

            Ok(())
        }
    }
}
