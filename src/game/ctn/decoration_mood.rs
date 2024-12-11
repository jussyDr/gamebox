//! Decoration mood.

use crate::Class;

/// Decoration mood.
#[derive(Default)]
pub struct DecorationMood;

impl Class for DecorationMood {
    const CLASS_ID: u32 = 0x0303a000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::{CurveSimpleNod, FxHdrScalesTech3},
        read::{
            readable,
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, Readable,
        },
    };

    use self::readable::{
        read_body_chunks, BodyChunk, BodyChunks, HeaderChunk, HeaderChunks, ReadBody,
    };

    use super::DecorationMood;

    impl Readable for DecorationMood {}

    impl readable::Sealed for DecorationMood {}

    impl HeaderChunks for DecorationMood {
        fn header_chunks<R: Read, I: IdStateMut, N>(
        ) -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

    impl ReadBody for DecorationMood {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for DecorationMood {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(0, Self::read_chunk_0),
                BodyChunk::normal(2, Self::read_chunk_2),
                BodyChunk::normal(4, Self::read_chunk_4),
                BodyChunk::normal(5, Self::read_chunk_5),
                BodyChunk::normal(7, Self::read_chunk_7),
                BodyChunk::normal(12, Self::read_chunk_12),
                BodyChunk::normal(15, Self::read_chunk_15),
                BodyChunk::normal(18, Self::read_chunk_18),
                BodyChunk::normal(19, Self::read_chunk_19),
            ]
            .into_iter()
        }
    }

    impl DecorationMood {
        fn read_chunk_0<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _latitude = r.f32()?;
            let _longitude = r.f32()?;
            let _delta_gmt = r.f32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _shadow_count_car_human = r.u32()?;
            let _shadow_count_car_opponent = r.u32()?;
            let _shadow_car_intensity = r.f32()?;
            let _shadow_scene = r.bool()?;
            let _background_is_locally_lighted = r.bool()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _lightmap = r.u32()?;

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _hms_ambiant_occ = r.u32()?;

            Ok(())
        }

        fn read_chunk_7<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.f32()?;
            r.f32()?;

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

        fn read_chunk_15(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.internal_node_ref::<CurveSimpleNod>()?;
            r.u32()?;
            r.internal_node_ref::<FxHdrScalesTech3>()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_18<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            r.string()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_19<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.f32()?;

            Ok(())
        }
    }
}
