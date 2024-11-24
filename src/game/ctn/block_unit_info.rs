use crate::{read::reader::ExternalNodeRef, Class, Nat3};

/// A block unit info.
#[derive(Default)]
pub struct BlockUnitInfo {
    relative_offset: Nat3,
    clips_north: Vec<ExternalNodeRef>,
    clips_east: Vec<ExternalNodeRef>,
    clips_south: Vec<ExternalNodeRef>,
    clips_west: Vec<ExternalNodeRef>,
    clips_top: Vec<ExternalNodeRef>,
    clips_bottom: Vec<ExternalNodeRef>,
}

impl Class for BlockUnitInfo {
    const CLASS_ID: u32 = 0x03036000;
}

mod read {
    use std::io::Read;

    use crate::{
        game::ctn::block_info_clip::BlockInfoClip,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody,
        },
    };

    use super::BlockUnitInfo;

    impl ReadBody for BlockUnitInfo {
        fn read_body<R: Read, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for BlockUnitInfo {
        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::new(0, Self::read_chunk_0),
                BodyChunk::new(1, Self::read_chunk_1),
                BodyChunk::skippable(2, Self::read_chunk_2),
                BodyChunk::new(4, Self::read_chunk_4),
                BodyChunk::new(5, Self::read_chunk_5),
                BodyChunk::new(7, Self::read_chunk_7),
                BodyChunk::new(12, Self::read_chunk_12),
                BodyChunk::new(13, Self::read_chunk_13),
            ]
            .into_iter()
        }
    }

    impl BlockUnitInfo {
        fn read_chunk_0(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let _place_pylons = r.u32()?;
            r.bool()?;
            r.bool()?;
            self.relative_offset = r.nat3()?;
            let _clips = r.list(|r| r.external_node_ref::<()>())?;

            Ok(())
        }

        fn read_chunk_1<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            r.id_or_null()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _underground = r.bool()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _accept_pylons = r.u32()?;

            Ok(())
        }

        fn read_chunk_5<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let _terrain_modifier_id = r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_7<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_12(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            let clip_count_bits = r.u32()?;
            let clip_count_north = (clip_count_bits & 0x00000007) as usize;
            let clip_count_east = ((clip_count_bits >> 3) & 0x00000007) as usize;
            let clip_count_south = ((clip_count_bits >> 6) & 0x00000007) as usize;
            let clip_count_west = ((clip_count_bits >> 9) & 0x00000007) as usize;
            let clip_count_top = ((clip_count_bits >> 12) & 0x00000007) as usize;
            let clip_count_bottom = ((clip_count_bits >> 15) & 0x00000007) as usize;

            self.clips_north =
                r.repeat(clip_count_north, |r| r.external_node_ref::<BlockInfoClip>())?;
            self.clips_east =
                r.repeat(clip_count_east, |r| r.external_node_ref::<BlockInfoClip>())?;
            self.clips_south =
                r.repeat(clip_count_south, |r| r.external_node_ref::<BlockInfoClip>())?;
            self.clips_west =
                r.repeat(clip_count_west, |r| r.external_node_ref::<BlockInfoClip>())?;
            self.clips_top =
                r.repeat(clip_count_top, |r| r.external_node_ref::<BlockInfoClip>())?;
            self.clips_bottom = r.repeat(clip_count_bottom, |r| {
                r.external_node_ref::<BlockInfoClip>()
            })?;
            r.u16()?;
            r.u16()?;

            Ok(())
        }

        fn read_chunk_13<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.byte_buf()?;

            Ok(())
        }
    }
}
