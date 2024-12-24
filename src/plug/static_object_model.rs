//! Static object model.

use crate::{Class, NodeRef};

use super::{solid_2_model::Solid2Model, surface::Surface};

/// Static object model.
#[derive(Default, Debug)]
pub struct StaticObjectModel {
    model: NodeRef<Solid2Model>,
    hit_shape: Option<HitShape>,
}

impl Class for StaticObjectModel {
    const CLASS_ID: u32 = 0x09159000;
}

impl StaticObjectModel {
    /// Model.
    pub const fn model(&self) -> &NodeRef<Solid2Model> {
        &self.model
    }

    /// Hit shape.
    pub const fn hit_shape(&self) -> Option<&HitShape> {
        self.hit_shape.as_ref()
    }
}

/// Hit shape.
#[derive(Debug)]
pub enum HitShape {
    /// Model.
    Model,
    /// Surface.
    Surface(NodeRef<Surface>),
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

    use super::{HitShape, StaticObjectModel};

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

            self.model = r.node_ref::<Solid2Model>()?;

            if !r.bool8()? {
                match r.node_ref_or_null::<Surface>()? {
                    Some(surface) => self.hit_shape = Some(HitShape::Surface(surface)),
                    None => self.hit_shape = Some(HitShape::Model),
                }
            }

            Ok(())
        }
    }
}
