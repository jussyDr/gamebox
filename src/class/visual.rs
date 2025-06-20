#[derive(Default)]
pub struct Visual;

mod read {
    use std::io::Read;

    use crate::{
        class::{vertex_stream::VertexStream, visual::Visual},
        read::{
            BodyChunk, BodyChunks, Error,
            reader::{IdsMut, NodesMut, Reader},
        },
    };

    impl BodyChunks for Visual {
        type Parent = Self;

        fn parent(&mut self) -> Option<&mut Self::Parent> {
            None
        }

        fn body_chunks<R: Read, I: IdsMut, N: NodesMut>()
        -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk {
                    id: 0x09006001,
                    read_fn: Self::read_chunk_1,
                    skippable: false,
                },
                BodyChunk {
                    id: 0x09006005,
                    read_fn: Self::read_chunk_5,
                    skippable: false,
                },
                BodyChunk {
                    id: 0x09006009,
                    read_fn: Self::read_chunk_9,
                    skippable: false,
                },
                BodyChunk {
                    id: 0x0900600b,
                    read_fn: Self::read_chunk_11,
                    skippable: false,
                },
                BodyChunk {
                    id: 0x0900600f,
                    read_fn: Self::read_chunk_15,
                    skippable: false,
                },
                BodyChunk {
                    id: 0x09006010,
                    read_fn: Self::read_chunk_16,
                    skippable: false,
                },
            ]
            .into_iter()
        }
    }

    impl Visual {
        fn read_chunk_1<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdsMut, N>,
        ) -> Result<(), Error> {
            r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_5<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let sub_visuals = r.list(|r| {
                r.u32()?;
                r.u32()?;
                r.u32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_9<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.f32()?;

            Ok(())
        }

        fn read_chunk_11<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let splits = r.list(|r| {
                r.u32()?;
                r.u32()?;
                let bounding_box = r.box3d()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_15(
            &mut self,
            r: &mut Reader<impl Read, impl IdsMut, impl NodesMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 6 {
                return Err(Error("unknown chunk version"));
            }

            let flags = r.u32()?;
            let num_texcoord_sets = r.u32()?;
            let count = r.u32()?;
            let vertex_streams = r.list(|r| r.internal_node_ref::<VertexStream>())?;
            let texcoord_sets: Vec<()> = r.repeat(num_texcoord_sets as usize, |r| todo!())?;
            let bounding_box = r.box3d()?;
            let bitmap_elem_to_packs: Vec<()> = r.list(|r| todo!())?;

            if version >= 5 {
                r.list(|r| r.u16())?;
            }

            if version >= 6 {
                r.u32()?;
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_16<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error("unknown chunk version"));
            }

            let morph_count = r.u32()?;

            Ok(())
        }
    }
}
