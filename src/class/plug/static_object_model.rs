//! Static object model.

use crate::{
    ClassId, NodeRef, SubExtensions,
    class::plug::{solid_2_model::Solid2Model, surface::Surface},
};

/// A model with a collidable hit shape.
#[derive(Default)]
pub struct StaticObjectModel {
    model: NodeRef<Solid2Model>,
    hit_shape: Option<NodeRef<Surface>>,
}

impl StaticObjectModel {
    /// The model.
    pub fn model(&self) -> &NodeRef<Solid2Model> {
        &self.model
    }

    /// Optional custom hit shape.
    /// If this returns `None` the hit shape is the same as the model.
    pub fn hit_shape(&self) -> &Option<NodeRef<Surface>> {
        &self.hit_shape
    }
}

impl ClassId for StaticObjectModel {
    const CLASS_ID: u32 = 0x09159000;
}

impl SubExtensions for StaticObjectModel {
    const SUB_EXTENSIONS: &[&str] = &["StaticObject"];
}

mod read {
    use crate::{
        class::plug::static_object_model::StaticObjectModel,
        read::{
            BodyReader, Error, HeaderChunk, HeaderChunks, HeaderReader, ReadBody, Readable,
            error_unknown_version,
        },
    };

    impl Readable for StaticObjectModel {}

    impl HeaderChunks for StaticObjectModel {
        fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>> {
            []
        }
    }

    impl ReadBody for StaticObjectModel {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 3 {
                return Err(error_unknown_version("static object model", version));
            }

            self.model = r.node_ref()?;
            self.hit_shape = if r.bool8()? {
                None
            } else {
                r.node_ref()? // Might be that there is no hit shape alltogether if this is `None`?
            };

            Ok(())
        }
    }
}
