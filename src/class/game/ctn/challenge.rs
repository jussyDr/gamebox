use std::marker::PhantomData;

use ouroboros::self_referencing;

use crate::read::{Error, NodeRefs, Readable};

pub struct Challenge(Inner);

#[self_referencing]
struct Inner {
    header_data: Vec<u8>,
    node_refs: NodeRefs,
    body_data: Vec<u8>,
    #[borrows(header_data)]
    #[covariant]
    header_chunks: HeaderChunks<'this>,
    #[borrows(node_refs, body_data)]
    #[covariant]
    body_chunks: BodyChunks<'this>,
}

struct HeaderChunks<'a> {
    marker: PhantomData<&'a ()>,
}

struct BodyChunks<'a> {
    marker: PhantomData<&'a ()>,
}

impl Readable for Challenge {
    fn read(header_data: Vec<u8>, node_refs: NodeRefs, body_data: Vec<u8>) -> Result<Self, Error> {
        let builder = InnerTryBuilder {
            header_data,
            node_refs,
            body_data,
            header_chunks_builder: |header_data| {
                Ok(HeaderChunks {
                    marker: PhantomData,
                })
            },
            body_chunks_builder: |node_refs, body_data| {
                Ok(BodyChunks {
                    marker: PhantomData,
                })
            },
        };

        builder.try_build().map(Self)
    }
}
