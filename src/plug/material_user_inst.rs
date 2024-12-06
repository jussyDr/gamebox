//! Material user inst.

use crate::Class;

/// A material user inst.
#[derive(PartialEq, Default, Debug)]
pub struct MaterialUserInst;

impl Class for MaterialUserInst {
    const CLASS_ID: u32 = 0x090fd000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::read::{
        read_body_chunks,
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error, ReadBody,
    };

    use super::MaterialUserInst;

    impl ReadBody for MaterialUserInst {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for MaterialUserInst {
        fn body_chunks<R: Read, I: IdStateMut, N>() -> impl Iterator<Item = BodyChunk<Self, R, I, N>>
        {
            [
                BodyChunk::normal(0, Self::read_chunk_0),
                BodyChunk::normal(1, Self::read_chunk_1),
                BodyChunk::normal(2, Self::read_chunk_2),
            ]
            .into_iter()
        }
    }

    impl MaterialUserInst {
        fn read_chunk_0<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 10 {
                return Err(Error::chunk_version(version));
            }

            let _material_name = r.id_or_null()?;
            let _model = r.id_or_null()?;
            let _base_texture = r.string()?;
            let _surface_physic_id = r.u8()?;
            let _surface_gameplay_id = r.u8()?;
            let _link = r.string()?;
            let _csts = r.list(|r| {
                r.id()?;
                r.id()?;
                r.u32()?;

                Ok(())
            })?;
            let _color = r.list(|r| r.u32())?;
            let _uv_anims = r.list(|r| {
                r.id()?;
                r.id()?;
                r.f32()?;
                r.u64()?;
                r.id()?;

                Ok(())
            })?;
            r.list(|r| r.id())?;
            let _user_textures = r.list(|r| {
                r.u32()?;
                let _texture = r.string()?;

                Ok(())
            })?;
            let _hiding_group = r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_1<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 5 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            let _tiling_u = r.u32()?;
            let _tiling_v = r.u32()?;
            let _texture_size_in_meters = r.f32()?;
            r.u32()?;
            let _is_natural = r.bool()?;

            Ok(())
        }

        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }
    }
}
