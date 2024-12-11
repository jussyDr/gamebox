//! Block item.

use std::sync::Arc;

use crate::{
    plug::{Crystal, StaticObjectModel},
    Class,
};

/// A block item.
#[derive(Default)]
pub struct BlockItem {
    archetype: Arc<str>,
    variants: Vec<BlockItemVariant>,
}

impl Class for BlockItem {
    const CLASS_ID: u32 = 0x2e025000;
}

impl BlockItem {
    /// Archetype block info identifier.
    pub const fn archetype(&self) -> &Arc<str> {
        &self.archetype
    }

    /// Variants.
    pub const fn variants(&self) -> &Vec<BlockItemVariant> {
        &self.variants
    }
}

/// Block item variant.
pub struct BlockItemVariant {
    index: u8,
    is_ground: bool,
    model: Option<BlockItemVariantModel>,
}

impl BlockItemVariant {
    /// Index.
    pub const fn index(&self) -> u8 {
        self.index
    }

    /// Is ground.
    pub const fn is_ground(&self) -> bool {
        self.is_ground
    }

    /// Model.
    pub const fn model(&self) -> Option<&BlockItemVariantModel> {
        self.model.as_ref()
    }
}

/// Block item variant model.
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
            [
                BodyChunk::normal(0, Self::read_chunk_0),
                BodyChunk::skippable(1, Self::read_chunk_1),
                BodyChunk::skippable(2, Self::read_chunk_2),
                BodyChunk::skippable(3, Self::read_chunk_3),
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
            let _archetype_block_info_collection_id = r.id()?;
            self.variants = r.list(|r| {
                let flags = r.u32()?;
                let index = ((flags >> 12) & 0x0000003f) as u8;
                let is_ground = flags & 0x01000000 != 0;
                let crystal = r.internal_node_ref_or_null::<Crystal>()?;

                Ok(BlockItemVariant {
                    index,
                    is_ground,
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
