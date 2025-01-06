//! Collector.

use std::sync::Arc;

use crate::{read::reader::FromVariant, Class, Rgba};

/// Collector.
#[derive(Clone, Default)]
pub struct Collector {
    page_name: String,
    catalog_position: u32,
    icon: Option<Icon>,
    id: Option<Arc<str>>,
    name: String,
    prod_state: ProdState,
    description: String,
}

impl Class for Collector {
    const CLASS_ID: u32 = 0x2e001000;
}

impl Collector {
    /// Icon.
    pub const fn icon(&self) -> Option<&Icon> {
        self.icon.as_ref()
    }

    /// Identifier.
    pub const fn id(&self) -> Option<&Arc<str>> {
        self.id.as_ref()
    }
}

/// Production state.
#[derive(Clone, Copy, Default, Debug)]
pub enum ProdState {
    /// Aborted.
    Aborted,
    /// Game box.
    GameBox,
    /// Dev build.
    DevBuild,
    /// Release.
    #[default]
    Release,
}

impl FromVariant<u8> for ProdState {
    fn from_variant(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Aborted),
            1 => Some(Self::GameBox),
            2 => Some(Self::DevBuild),
            3 => Some(Self::Release),
            _ => None,
        }
    }
}

/// Collector icon.
#[derive(Clone)]
pub enum Icon {
    /// Normal icon.
    Normal {
        /// Data.
        data: Vec<Rgba>,
    },
    /// WebP icon.
    WebP {
        /// WebP file data.
        data: Vec<u8>,
    },
}

mod read {
    use std::io::Read;

    use crate::read::{
        readable::{HeaderChunk, HeaderChunks},
        reader::{IdStateMut, NodeStateMut, Reader},
        BodyChunk, BodyChunks, Error,
    };

    use super::{Collector, Icon};

    impl HeaderChunks for Collector {
        fn header_chunks<R: Read, I: IdStateMut, N>(
        ) -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [
                HeaderChunk::new(3, Self::read_chunk_3),
                HeaderChunk::new(4, Self::read_chunk_4),
                HeaderChunk::new(6, Self::read_chunk_6),
            ]
            .into_iter()
        }
    }

    impl BodyChunks for Collector {
        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(9, Self::read_chunk_9),
                BodyChunk::normal(11, Self::read_chunk_11),
                BodyChunk::normal(12, Self::read_chunk_12),
                BodyChunk::normal(13, Self::read_chunk_13),
                BodyChunk::normal(14, Self::read_chunk_14),
                BodyChunk::normal(16, Self::read_chunk_16),
                BodyChunk::normal(17, Self::read_chunk_17),
                BodyChunk::normal(18, Self::read_chunk_18),
            ]
            .into_iter()
        }
    }

    impl Collector {
        fn read_chunk_3<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            self.id = r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            let version = r.u32()?;

            if version != 8 {
                return Err(Error::chunk_version(version));
            }

            self.page_name = r.string()?;
            r.id_or_null()?;
            let _flags = r.u32()?;
            self.catalog_position = r.u16()? as u32;
            self.name = r.string()?;
            self.prod_state = r.enum_u8()?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let icon_width = r.u16()?;
            let icon_height = r.u16()?;

            let is_webp = icon_width & 0x8000 != 0 && icon_height & 0x8000 != 0;

            self.icon = Some(if is_webp {
                r.u16()?;
                let data = r.byte_buf()?;

                Icon::WebP { data }
            } else {
                let data =
                    r.repeat((icon_width as usize) + (icon_height as usize), |r| r.rgba())?;

                Icon::Normal { data }
            });

            Ok(())
        }

        fn read_chunk_6<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _lightmap_compute_time = r.u64()?;

            Ok(())
        }

        fn read_chunk_9(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            self.page_name = r.string()?;

            if r.bool()? {
                r.external_node_ref::<()>()?;
            }

            r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_11<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            self.id = r.id_or_null()?;
            r.id_or_null()?;
            r.id()?;

            Ok(())
        }

        fn read_chunk_12<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.name = r.string()?;

            Ok(())
        }

        fn read_chunk_13<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.description = r.string()?;

            Ok(())
        }

        fn read_chunk_14<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _icon_use_auto_render = r.bool()?;
            let _icon_quarter_rotation_y = r.u32()?;

            Ok(())
        }

        fn read_chunk_16<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 3 | 4) {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            let _skin_directory = r.string_or_empty()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_17<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let _is_internal = r.bool()?;
            let _is_advanced = r.bool()?;
            self.catalog_position = r.u32()?;
            self.prod_state = r.enum_u8()?;

            Ok(())
        }

        fn read_chunk_18<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
