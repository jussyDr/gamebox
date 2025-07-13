use std::marker::PhantomData;

use ouroboros::self_referencing;

use crate::read::{Error, NodeRefs, Readable};

pub struct Prefab(Inner);

#[self_referencing]
struct Inner {
    node_refs: NodeRefs,
    body_data: Vec<u8>,
    #[borrows(node_refs, body_data)]
    #[covariant]
    body: Body<'this>,
}

struct Body<'a> {
    marker: PhantomData<&'a ()>,
}

impl Readable for Prefab {
    fn read(header_data: Vec<u8>, node_refs: NodeRefs, body_data: Vec<u8>) -> Result<Self, Error> {
        let builder = InnerTryBuilder {
            node_refs,
            body_data,
            body_builder: |node_refs, body_data| {
                Ok(Body {
                    marker: PhantomData,
                })
            },
        };

        builder.try_build().map(Self)
    }
}
