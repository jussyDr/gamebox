//! Veget tree model.

use crate::Class;

/// Veget tree model.
#[derive(Default)]
pub struct VegetTreeModel;

impl Class for VegetTreeModel {
    const CLASS_ID: u32 = 0x2f086000;
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::{Bitmap, VisualIndexedTriangles},
        read::{
            readable,
            reader::{IdStateMut, NodeStateMut, Reader},
            Error, Readable,
        },
    };

    use self::readable::{HeaderChunk, HeaderChunks, ReadBody};

    use super::VegetTreeModel;

    impl Readable for VegetTreeModel {}

    impl readable::Sealed for VegetTreeModel {}

    impl HeaderChunks for VegetTreeModel {
        fn header_chunks<R, I, N>() -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

    impl ReadBody for VegetTreeModel {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 21 {
                return Err(Error::version("veget tree model", version));
            }

            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            let _color_image = r.external_node_ref::<()>()?;
            let _normal_image = r.external_node_ref::<()>()?;
            let _roughness_image = r.external_node_ref::<()>()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.u8()?;
            let _color_texture = r.external_node_ref::<Bitmap>()?;
            let _normal_teture = r.external_node_ref::<Bitmap>()?;
            let _roughness_texture = r.external_node_ref::<Bitmap>()?;
            let _color_image = r.external_node_ref::<()>()?;
            let _normal_image = r.external_node_ref::<()>()?;
            let _roughness_image = r.external_node_ref::<()>()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.u8()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            let _color_image = r.external_node_ref::<()>()?;
            let _normal_image = r.external_node_ref::<()>()?;
            let _roughness_image = r.external_node_ref::<()>()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.id_or_null()?;
            r.u8()?;
            r.u32()?;
            r.u32()?;
            r.id()?;
            r.id()?;
            r.id()?;
            r.u32()?;
            r.u16()?;
            r.internal_node_ref::<VisualIndexedTriangles>()?;
            r.u8()?;
            r.u16()?;
            r.internal_node_ref::<VisualIndexedTriangles>()?;
            r.u8()?;
            r.u16()?;
            r.internal_node_ref::<VisualIndexedTriangles>()?;
            r.u8()?;
            r.u16()?;
            r.internal_node_ref::<VisualIndexedTriangles>()?;
            r.u8()?;
            r.u16()?;
            r.u32()?;
            r.internal_node_ref::<VisualIndexedTriangles>()?;
            r.u8()?;
            r.u16()?;
            r.internal_node_ref::<VisualIndexedTriangles>()?;
            r.u8()?;
            r.u16()?;
            r.internal_node_ref::<VisualIndexedTriangles>()?;
            r.u8()?;
            r.u16()?;
            r.internal_node_ref::<VisualIndexedTriangles>()?;
            r.u8()?;
            r.u16()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u8()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.list(|r| r.vec3::<f32>())?;
            r.list(|r| {
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;

                Ok(())
            })?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.u32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
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
