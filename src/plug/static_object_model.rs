//! Static object model.

use crate::{Class, NodeRef};

use super::{solid_2_model::Solid2Model, surface::Surface};

/// Static object model.
#[derive(Default, Debug)]
pub struct StaticObjectModel {
    mesh: NodeRef<Solid2Model>,
    is_collidable: bool,
    hit_shape: Option<NodeRef<Surface>>,
}

impl Class for StaticObjectModel {
    const CLASS_ID: u32 = 0x09159000;
}

impl StaticObjectModel {
    /// Mesh of the model.
    pub const fn mesh(&self) -> &NodeRef<Solid2Model> {
        &self.mesh
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::{solid_2_model::Solid2Model, surface::Surface},
        read::{
            readable::{HeaderChunk, HeaderChunks, Sealed},
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, ReadBody, Readable,
        },
    };

    use super::StaticObjectModel;

    impl Readable for StaticObjectModel {}

    impl Sealed for StaticObjectModel {}

    impl HeaderChunks for StaticObjectModel {
        fn header_chunks<R, I, N>() -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

    impl ReadBody for StaticObjectModel {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(Error::chunk_version(version));
            }

            self.mesh = r.node_ref::<Solid2Model>()?;
            self.is_collidable = r.bool8()?;

            if !self.is_collidable {
                self.hit_shape = r.node_ref_or_null::<Surface>()?;
            }

            Ok(())
        }
    }
}
