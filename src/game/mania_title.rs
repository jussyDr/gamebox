//! Mania title.

use std::ops::Deref;

use crate::{read::reader::ExternalNodeRef, Class};

use super::Nod;

/// Mania title.
#[derive(Default)]
pub struct ManiaTitle {
    parent: Nod,
    collections: Vec<ExternalNodeRef>,
}

impl Class for ManiaTitle {
    const CLASS_ID: u32 = 0x03001000;
}

impl Deref for ManiaTitle {
    type Target = Nod;

    fn deref(&self) -> &Nod {
        &self.parent
    }
}

impl ManiaTitle {
    /// Collections.
    pub const fn collections(&self) -> &Vec<ExternalNodeRef> {
        &self.collections
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::{
            ctn::{Challenge, Collection},
            skinned_nod::SkinnedNod,
        },
        read::{
            readable,
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, Readable,
        },
    };

    use self::readable::{
        read_body_chunks, BodyChunk, BodyChunks, HeaderChunk, HeaderChunks, ReadBody,
    };

    use super::ManiaTitle;

    impl Readable for ManiaTitle {}

    impl readable::Sealed for ManiaTitle {}

    impl HeaderChunks for ManiaTitle {
        fn header_chunks<R, I, N>() -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

    impl ReadBody for ManiaTitle {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for ManiaTitle {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(0, Self::read_chunk_0),
                BodyChunk::normal(1, Self::read_chunk_1),
                BodyChunk::normal(2, Self::read_chunk_2),
                BodyChunk::normal(3, Self::read_chunk_3),
                BodyChunk::normal(5, Self::read_chunk_5),
                BodyChunk::normal(6, Self::read_chunk_6),
                BodyChunk::normal(8, Self::read_chunk_8),
                BodyChunk::normal(10, Self::read_chunk_10),
                BodyChunk::normal(11, Self::read_chunk_11),
                BodyChunk::normal(12, Self::read_chunk_12),
            ]
            .into_iter()
        }
    }

    impl ManiaTitle {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.string()?;
            r.u32()?;
            r.string()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_1(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.u32()?;
            let _menu_background_image = r.external_node_ref::<()>()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            let _menu_header_image = r.external_node_ref::<()>()?;
            r.u32()?;
            r.internal_node_ref::<SkinnedNod>()?;
            let _menu_manialink = r.external_node_ref::<()>()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_2<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.u32()?;
            let _title_core = r.external_node_ref::<()>()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_3<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.u32()?;
            self.collections = r.list(|r| r.external_node_ref::<Collection>())?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_5(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.internal_node_ref::<SkinnedNod>()?;

            Ok(())
        }

        fn read_chunk_6<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            r.id()?;
            r.u32()?;
            r.id()?;

            Ok(())
        }

        fn read_chunk_8<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            let _base_map = r.external_node_ref::<Challenge>()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_10<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_11<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_12<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
