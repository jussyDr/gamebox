//! Collection.

use std::sync::Arc;

use crate::Class;

/// Collection.
#[derive(Default)]
pub struct Collection {
    square_size: f32,
    square_height: f32,
    vehicle_id: Arc<str>,
    block_info_folder: String,
    item_folder: Option<String>,
    decoration_folder: String,
}

impl Class for Collection {
    const CLASS_ID: u32 = 0x03033000;
}

impl Collection {
    /// Challenge qquare size.
    pub const fn square_size(&self) -> f32 {
        self.square_size
    }

    /// Challenge square height.
    pub const fn square_height(&self) -> f32 {
        self.square_height
    }

    /// Block info folder.
    pub const fn block_info_folder(&self) -> &String {
        &self.block_info_folder
    }

    /// Item folder.
    pub const fn item_folder(&self) -> Option<&String> {
        self.item_folder.as_ref()
    }

    /// Decoration folder.
    pub const fn decoration_folder(&self) -> &String {
        &self.decoration_folder
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        function::Shader,
        game::ctn::Zone,
        plug::Bitmap,
        read::{
            readable,
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, Readable,
        },
    };

    use self::readable::{
        read_body_chunks, BodyChunk, BodyChunks, HeaderChunk, HeaderChunks, ReadBody,
    };

    use super::Collection;

    impl Readable for Collection {}

    impl readable::Sealed for Collection {}

    impl HeaderChunks for Collection {
        fn header_chunks<R, I, N>() -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

    impl ReadBody for Collection {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Collection {
        fn body_chunks<R: Read, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(9, Self::read_chunk_9),
                BodyChunk::normal(12, Self::read_chunk_12),
                BodyChunk::normal(13, Self::read_chunk_13),
                BodyChunk::normal(14, Self::read_chunk_14),
                BodyChunk::normal(17, Self::read_chunk_17),
                BodyChunk::normal(25, Self::read_chunk_25),
                BodyChunk::normal(26, Self::read_chunk_26),
                BodyChunk::normal(29, Self::read_chunk_29),
                BodyChunk::normal(31, Self::read_chunk_31),
                BodyChunk::normal(32, Self::read_chunk_32),
                BodyChunk::normal(33, Self::read_chunk_33),
                BodyChunk::normal(39, Self::read_chunk_39),
                BodyChunk::normal(40, Self::read_chunk_40),
                BodyChunk::normal(41, Self::read_chunk_41),
                BodyChunk::normal(42, Self::read_chunk_42),
                BodyChunk::normal(47, Self::read_chunk_47),
                BodyChunk::normal(48, Self::read_chunk_48),
                BodyChunk::normal(49, Self::read_chunk_49),
                BodyChunk::normal(51, Self::read_chunk_51),
                BodyChunk::normal(52, Self::read_chunk_52),
                BodyChunk::normal(54, Self::read_chunk_54),
                BodyChunk::normal(55, Self::read_chunk_55),
                BodyChunk::normal(56, Self::read_chunk_56),
                BodyChunk::normal(57, Self::read_chunk_57),
                BodyChunk::normal(58, Self::read_chunk_58),
                BodyChunk::normal(59, Self::read_chunk_59),
                BodyChunk::normal(60, Self::read_chunk_60),
                BodyChunk::normal(61, Self::read_chunk_61),
                BodyChunk::normal(62, Self::read_chunk_62),
                BodyChunk::normal(63, Self::read_chunk_63),
                BodyChunk::normal(64, Self::read_chunk_64),
                BodyChunk::normal(65, Self::read_chunk_65),
                BodyChunk::normal(66, Self::read_chunk_66),
                BodyChunk::normal(67, Self::read_chunk_67),
            ]
            .into_iter()
        }
    }

    impl Collection {
        fn read_chunk_9(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let _collection = r.id_or_null()?;
            let _complete_list_zone_list =
                r.list_with_version(|r| r.external_node_ref::<Zone>())?;
            let _default_zone = r.external_node_ref::<Zone>()?;
            let _need_unlock = r.bool()?;
            self.square_size = r.f32()?;
            self.square_height = r.f32()?;
            self.vehicle_id = r.id()?;
            let _vehicle_collection = r.id_or_null()?;
            let _vehicle_author = r.id()?;

            Ok(())
        }

        fn read_chunk_12<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _is_editable = r.bool()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_13<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodeStateMut>,
        ) -> Result<(), Error> {
            if r.bool()? {
                let _icon = r.external_node_ref::<Bitmap>()?;
            }

            if r.bool()? {
                let _icon = r.external_node_ref::<Bitmap>()?;
            }

            Ok(())
        }

        fn read_chunk_14<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_17<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_25<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let _load_screen = r.external_node_ref::<Bitmap>()?;

            Ok(())
        }

        fn read_chunk_26<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
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
            r.f32()?;
            r.string()?;

            Ok(())
        }

        fn read_chunk_29<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodeStateMut>,
        ) -> Result<(), Error> {
            r.u32()?;
            r.list_with_version(|r| r.external_node_ref::<()>())?;

            Ok(())
        }

        fn read_chunk_31<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_32<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            self.block_info_folder = r.string()?;
            self.item_folder = r.string_or_empty()?;
            self.decoration_folder = r.string()?;
            let _folder_menus_items = r.string()?;

            Ok(())
        }

        fn read_chunk_33<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _display_name = r.string()?;

            Ok(())
        }

        fn read_chunk_39<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _board_square_height = r.f32()?;
            let _board_square_width = r.f32()?;

            Ok(())
        }

        fn read_chunk_40<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _item_additional_folder = r.string()?;

            Ok(())
        }

        fn read_chunk_41<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _item_additional_folder = r.string()?;

            Ok(())
        }

        fn read_chunk_42<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _folder_decal_models = r.string()?;

            Ok(())
        }

        fn read_chunk_47<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _tunnel_specular = r.vec3()?;

            Ok(())
        }

        fn read_chunk_48<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_49<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _macro_decals_folder = r.string()?;

            Ok(())
        }

        fn read_chunk_51<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            r.list(|r| r.id())?;
            let _decal_fade = r.u32()?;

            Ok(())
        }

        fn read_chunk_52<I>(
            &mut self,
            r: &mut Reader<impl Read, I, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            let _clouds_shader = r.external_node_ref::<Shader>()?;
            let _clouds_texture = r.external_node_ref::<Bitmap>()?;
            let _env_layer_dirt = r.external_node_ref::<Bitmap>()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_54<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;

            Ok(())
        }

        fn read_chunk_55<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.f32()?;

            Ok(())
        }

        fn read_chunk_56(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 8 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            r.u32()?;
            r.id()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.u32()?;
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

            Ok(())
        }

        fn read_chunk_57<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 18 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;

            Ok(())
        }

        fn read_chunk_58<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            r.u32()?;
            r.u32()?;
            r.bool()?;
            r.f32()?;
            r.u32()?;
            r.f32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_59<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.u8()?;
            r.u8()?;
            r.u8()?;
            r.u8()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_60<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            let _spectators_folder = r.string()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_61<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_62<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_63<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_64<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_65<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.string()?;

            Ok(())
        }

        fn read_chunk_66<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;

            Ok(())
        }

        fn read_chunk_67<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let _modifier_folder = r.string()?;

            Ok(())
        }
    }
}
