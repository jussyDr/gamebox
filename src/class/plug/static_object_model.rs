use std::{borrow::Cow, marker::PhantomData};

use ouroboros::self_referencing;

use crate::read::{Error, NodeRefs, Readable};

pub struct StaticObjectModel<'a>(Inner<'a>);

#[self_referencing]
struct Inner<'a> {
    node_refs: Cow<'a, NodeRefs>,
    body_data: Cow<'a, [u8]>,
    #[borrows(node_refs, body_data)]
    #[covariant]
    body: Body<'this>,
}

struct Body<'a> {
    marker: PhantomData<&'a ()>,
}

impl Readable for StaticObjectModel<'_> {
    fn read(header_data: Vec<u8>, node_refs: NodeRefs, body_data: Vec<u8>) -> Result<Self, Error> {
        let builder = InnerTryBuilder {
            node_refs: Cow::Owned(node_refs),
            body_data: Cow::Owned(body_data),
            body_builder: |node_refs, body_data| {
                Ok(Body {
                    marker: PhantomData,
                })
            },
        };

        builder.try_build().map(Self)
    }
}
