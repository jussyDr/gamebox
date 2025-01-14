//! Block item.

use std::sync::Arc;

use crate::{
    plug::{Crystal, StaticObjectModel},
    Class,
};

/// A block item.
#[derive(Default, Debug)]
pub struct BlockItem {
    archetype: Arc<str>,
    variants: Vec<BlockItemVariant>,
}

impl Class for BlockItem {
    const CLASS_ID: u32 = 0x2e025000;
}

impl BlockItem {
    /// Archetype.
    pub const fn archetype(&self) -> &Arc<str> {
        &self.archetype
    }

    /// Variants.
    pub const fn variants(&self) -> &Vec<BlockItemVariant> {
        &self.variants
    }
}

/// Block item variant.
#[derive(Debug)]
pub struct BlockItemVariant {
    flags: u32,
    model: Option<BlockItemVariantModel>,
}

impl BlockItemVariant {
    /// Flags.
    pub const fn flags(&self) -> u32 {
        self.flags
    }

    /// Model.
    pub const fn model(&self) -> Option<&BlockItemVariantModel> {
        self.model.as_ref()
    }
}

/// Block item variant model.
#[derive(Debug)]
pub enum BlockItemVariantModel {
    /// Crystal.
    Crystal(Arc<Crystal>),
    /// Static object.
    StaticObject(Arc<StaticObjectModel>),
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::{Crystal, StaticObjectModel},
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::{BlockItem, BlockItemVariant, BlockItemVariantModel};

    impl ReadBody for BlockItem {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for BlockItem {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            #![allow(clippy::redundant_closure)]
            [
                BodyChunk::normal(0, Self::read_chunk_0),
                BodyChunk::skippable(1, |s, r| Self::read_chunk_1(s, r)),
                BodyChunk::skippable(2, |s, r| Self::read_chunk_2(s, r)),
                BodyChunk::skippable(3, |s, r| Self::read_chunk_3(s, r)),
            ]
            .into_iter()
        }
    }

    impl BlockItem {
        fn read_chunk_0(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 0 | 1) {
                return Err(Error::chunk_version(version));
            }

            self.archetype = r.id()?;
            let _archetype_collection = r.id_or_null()?;
            self.variants = r.list(|r| {
                let flags = r.u32()?;
                let crystal = r.internal_node_ref_or_null::<Crystal>()?;

                Ok(BlockItemVariant {
                    flags,
                    model: crystal.map(BlockItemVariantModel::Crystal),
                })
            })?;

            if version >= 1 && r.bool8()? {
                for variant in &mut self.variants {
                    let flags = r.u8()?;

                    if flags & 0x01 != 0 {
                        if let Some(static_object) =
                            r.internal_node_ref_or_null::<StaticObjectModel>()?
                        {
                            variant.model = Some(BlockItemVariantModel::StaticObject(static_object))
                        }
                    }
                }
            }

            Ok(())
        }

        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_3<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            for _ in 0..self.variants.len() {
                r.u8()?;
            }

            Ok(())
        }
    }
}
