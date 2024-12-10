//! Decoration.

use std::ops::Deref;

use crate::{read::reader::ExternalNodeRef, Class};

use super::collector::Collector;

/// Decoration of a challenge.
#[derive(Default)]
pub struct Decoration {
    parent: Collector,
    size: ExternalNodeRef,
    mood: ExternalNodeRef,
    audio: ExternalNodeRef,
    map: Option<ExternalNodeRef>,
}

impl Class for Decoration {
    const CLASS_ID: u32 = 0x03038000;
}

impl Deref for Decoration {
    type Target = Collector;

    fn deref(&self) -> &Collector {
        &self.parent
    }
}

impl Decoration {
    /// Decoration size.
    pub const fn size(&self) -> &ExternalNodeRef {
        &self.size
    }

    /// Decoration mood.
    pub const fn mood(&self) -> &ExternalNodeRef {
        &self.mood
    }

    /// Decoration audio.
    pub const fn audio(&self) -> &ExternalNodeRef {
        &self.audio
    }

    /// Decoration map.
    pub const fn map(&self) -> Option<&ExternalNodeRef> {
        self.map.as_ref()
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        game::ctn::challenge::Challenge,
        read::{
            read_body_chunks,
            readable::{HeaderChunk, HeaderChunks, Sealed},
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody, Readable,
        },
    };

    use super::Decoration;

    impl Readable for Decoration {}

    impl Sealed for Decoration {}

    impl HeaderChunks for Decoration {
        fn header_chunks<R, I, N>() -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

    impl ReadBody for Decoration {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Decoration {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(17, Self::read_chunk_17),
                BodyChunk::normal(19, Self::read_chunk_19),
                BodyChunk::normal(20, Self::read_chunk_20),
                BodyChunk::normal(21, Self::read_chunk_21),
                BodyChunk::normal(22, Self::read_chunk_22),
                BodyChunk::normal(23, Self::read_chunk_23),
                BodyChunk::normal(24, Self::read_chunk_24),
                BodyChunk::normal(25, Self::read_chunk_25),
                BodyChunk::normal(26, Self::read_chunk_26),
                BodyChunk::normal(27, Self::read_chunk_27),
            ]
            .into_iter()
        }
    }

    impl Decoration {
        fn read_chunk_17(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.size = r.external_node_ref::<()>()?;

            Ok(())
        }

        fn read_chunk_19(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.mood = r.external_node_ref::<()>()?;

            Ok(())
        }

        fn read_chunk_20(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let _decorator_solid_warp = r.external_node_ref_or_null::<()>()?;

            Ok(())
        }

        fn read_chunk_21(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let _terrain_modifier_covered = r.external_node_ref_or_null::<()>()?;

            Ok(())
        }

        fn read_chunk_22(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let _terrain_modifier_base = r.external_node_ref_or_null::<()>()?;

            Ok(())
        }

        fn read_chunk_23<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let _decoration_zone_frontier_id = r.id_or_null()?;
            let _is_water_outside_playfield = r.bool()?;

            Ok(())
        }

        fn read_chunk_24<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            let _vehicle_fx_skin = r.u32()?;
            let _vehicle_fx_folder = r.string()?;

            Ok(())
        }

        fn read_chunk_25(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            self.audio = r.external_node_ref::<()>()?;
            let _deco_audio_ambient = r.u32()?;

            Ok(())
        }

        fn read_chunk_26(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let _deco_material_modifiers = r.u32()?;

            Ok(())
        }

        fn read_chunk_27(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            self.map = r.external_node_ref_or_null::<Challenge>()?;
            let _deco_map_lightmap = r.u32()?;

            Ok(())
        }
    }
}
