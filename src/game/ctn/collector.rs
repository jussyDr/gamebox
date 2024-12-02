//! Collector.

use crate::Class;

/// A collector.
#[derive(PartialEq, Default, Debug)]
pub struct Collector;

impl Class for Collector {
    const CLASS_ID: u32 = 0x2e001000;
}

mod read {
    use std::io::Read;

    use crate::read::{
        reader::{IdStateMut, Reader},
        BodyChunk, BodyChunks, Error,
    };

    use super::Collector;

    impl BodyChunks for Collector {
        fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>>
        {
            [
                BodyChunk::new(9, Self::read_chunk_9),
                BodyChunk::new(11, Self::read_chunk_11),
                BodyChunk::new(12, Self::read_chunk_12),
                BodyChunk::new(13, Self::read_chunk_13),
                BodyChunk::new(14, Self::read_chunk_14),
                BodyChunk::new(16, Self::read_chunk_16),
                BodyChunk::new(17, Self::read_chunk_17),
                BodyChunk::new(18, Self::read_chunk_18),
            ]
            .into_iter()
        }
    }

    impl Collector {
        fn read_chunk_9<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let _page_name = r.string()?;

            if r.bool()? {
                todo!()
            }

            r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_11<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            r.id_or_null()?;
            r.id()?;
            r.id()?;

            Ok(())
        }

        fn read_chunk_12<N, I>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _name = r.string()?;

            Ok(())
        }

        fn read_chunk_13<N, I>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _description = r.string()?;

            Ok(())
        }

        fn read_chunk_14<N, I>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _icon_use_auto_render = r.bool()?;
            let _icon_quarter_rotation_y = r.u32()?;

            Ok(())
        }

        fn read_chunk_16<N, I>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 3 | 4) {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            let _skin_directory = r.string()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_17<N, I>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let _is_internal = r.bool()?;
            let _is_advanced = r.bool()?;
            let _catalog_position = r.u32()?;
            let _prod_state = r.u8()?;

            Ok(())
        }

        fn read_chunk_18<N, I>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
