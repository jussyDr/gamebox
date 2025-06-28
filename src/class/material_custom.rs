use crate::Class;

/// A custom material.
#[derive(Default)]
pub struct MaterialCustom;

impl Class for MaterialCustom {
    const CLASS_ID: u32 = 0x0903a000;
}

mod read {
    use std::io::Read;

    use crate::{
        class::material_custom::MaterialCustom,
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, read_body_chunks,
            reader::{IdTableRef, NodeTableRef, Reader},
        },
    };

    impl ReadBody for MaterialCustom {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), crate::read::Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for MaterialCustom {
        fn body_chunks<R: Read, I: IdTableRef, N: NodeTableRef>()
        -> impl IntoIterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::new(0x0903a004, Self::read_chunk_4),
                BodyChunk::new(0x0903a00a, Self::read_chunk_10),
                BodyChunk::new(0x0903a00c, Self::read_chunk_12),
                BodyChunk::skippable(0x0903a00f, Self::read_chunk_15),
                BodyChunk::skippable(0x0903a011, Self::read_chunk_17),
                BodyChunk::new(0x0903a012, Self::read_chunk_18),
                BodyChunk::new(0x0903a013, Self::read_chunk_19),
                BodyChunk::new(0x0903a014, Self::read_chunk_20),
                BodyChunk::new(0x0903a015, Self::read_chunk_21),
                BodyChunk::new(0x0903a016, Self::read_chunk_22),
            ]
        }
    }

    impl MaterialCustom {
        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.list(|r| r.u32())?;

            Ok(())
        }

        fn read_chunk_10<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, N>,
        ) -> Result<(), Error> {
            let gpu_fxs1 = r.list(|r| {
                r.id()?;
                let count1 = r.u32()?;
                let count2 = r.u32()?;
                r.bool32()?;

                for _ in 0..count2 {
                    r.repeat(count1 as usize, |r| r.f32())?;
                }

                Ok(())
            })?;
            let gpu_fxs2 = r.list(|r| {
                r.id()?;
                let count1 = r.u32()?;
                let count2 = r.u32()?;
                r.bool32()?;

                for _ in 0..count2 {
                    r.repeat(count1 as usize, |r| r.f32())?;
                }

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_12<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, N>,
        ) -> Result<(), Error> {
            let skip_samplers = r.list(|r| {
                let name = r.id()?;
                r.bool32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_15<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error("unknown chunk version".into()));
            }

            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.list(|r| {
                r.id()?;
                r.u32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_17<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_18<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_19(
            &mut self,
            r: &mut Reader<impl Read, impl IdTableRef, impl NodeTableRef>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error("unknown chunk version".into()));
            }

            let textures = r.list(|r| {
                let name = r.id()?;
                r.u32()?;
                let texture = r.external_node_ref()?;
                r.u32()?;
                r.u32()?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_20<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error("unknown chunk version".into()));
            }

            r.list(|r| {
                r.u32()?;
                let len = r.u32()?;
                r.bytes(len as usize)?;

                Ok(())
            })?;

            Ok(())
        }

        fn read_chunk_21<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error("unknown chunk version".into()));
            }

            if r.u32()? == 0 {
                r.string()?;
                r.string()?;
                r.u32()?;
                r.u32()?;
            }

            Ok(())
        }

        fn read_chunk_22<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
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
