use crate::ClassId;

/// A collector.
#[derive(Default)]
pub struct Collector;

impl ClassId for Collector {
    const CLASS_ID: u32 = 0x2e001000;
}

mod read {
    use crate::{
        Delme,
        class::game_data::collector::Collector,
        read::{BodyChunk, BodyChunks, Error, error_unknown_chunk_version, reader::BodyReader},
    };

    impl BodyChunks for Collector {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(9, Self::read_chunk_9),
                BodyChunk::new(11, Self::read_chunk_11),
                BodyChunk::new(12, Self::read_chunk_12),
                BodyChunk::new(13, Self::read_chunk_13),
                BodyChunk::new(16, Self::read_chunk_16),
                BodyChunk::new(17, Self::read_chunk_17),
                BodyChunk::new(18, Self::read_chunk_18),
            ]
        }
    }

    impl Collector {
        fn read_chunk_9(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let page_name = r.string()?;

            if r.bool32()? {
                todo!()
            }

            let parent_collector_id = r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_11(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.id()?;
            r.id()?;
            r.id()?;

            Ok(())
        }

        fn read_chunk_12(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let name = r.string()?;

            Ok(())
        }

        fn read_chunk_13(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let description = r.string()?;

            Ok(())
        }

        fn read_chunk_16(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 4 {
                return Err(error_unknown_chunk_version(version));
            }

            let default_skin = r.external_node_ref_or_null::<Delme>()?;
            let skin_directory = r.string()?;

            if skin_directory.is_empty() {
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_17(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(error_unknown_chunk_version(version));
            }

            let is_internal = r.bool32()?;
            let is_advanced = r.bool32()?;
            let catalog_position = r.u32()?;
            let prod_state = r.u8()?;

            Ok(())
        }

        fn read_chunk_18(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
