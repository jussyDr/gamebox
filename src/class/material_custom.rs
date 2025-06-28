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
    }
}
