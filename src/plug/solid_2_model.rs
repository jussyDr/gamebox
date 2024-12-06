//! Solid 2 model.

use std::sync::Arc;

use crate::{read::reader::ExternalNodeRef, Class};

use super::visual_indexed_triangles::VisualIndexedTriangles;

/// A solid 2 model.
#[derive(Default)]
pub struct Solid2Model {
    shaded_geoms: Vec<ShadedGeom>,
    visuals: Vec<Arc<VisualIndexedTriangles>>,
    materials: Vec<ExternalNodeRef>,
}

impl Class for Solid2Model {
    const CLASS_ID: u32 = 0x090bb000;
}

impl Solid2Model {
    pub const fn shaded_geoms(&self) -> &Vec<ShadedGeom> {
        &self.shaded_geoms
    }

    pub const fn visuals(&self) -> &Vec<Arc<VisualIndexedTriangles>> {
        &self.visuals
    }

    pub const fn materials(&self) -> &Vec<ExternalNodeRef> {
        &self.materials
    }
}

pub struct ShadedGeom {
    visual_index: u32,
    material_index: u32,
}

impl ShadedGeom {
    pub const fn visual_index(&self) -> u32 {
        self.visual_index
    }

    pub const fn material_index(&self) -> u32 {
        self.material_index
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::{
            light::Light, material::Material, visual_indexed_triangles::VisualIndexedTriangles,
        },
        read::{
            read_body_chunks,
            readable::Sealed,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody, Readable,
        },
    };

    use super::{ShadedGeom, Solid2Model};

    impl Readable for Solid2Model {}

    impl Sealed for Solid2Model {}

    impl ReadBody for Solid2Model {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Solid2Model {
        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(0, Self::read_chunk_0),
                BodyChunk::skippable(2, Self::read_chunk_2),
            ]
            .into_iter()
        }
    }

    impl Solid2Model {
        fn read_chunk_0(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 34 {
                return Err(Error::chunk_version(version));
            }

            r.id_or_null()?;
            self.shaded_geoms = r.list(|r| {
                let visual_index = r.u32()?;
                let material_index = r.u32()?;
                r.u32()?;
                let _lod = r.u32()?;
                r.u32()?;

                Ok(ShadedGeom {
                    visual_index,
                    material_index,
                })
            })?;
            self.visuals =
                r.list_with_version(|r| r.internal_node_ref::<VisualIndexedTriangles>())?;
            let _material_ids = r.list(|r| r.id())?;
            let material_count = r.u32()?;

            if material_count == 0 {
                self.materials = r.list_with_version(|r| r.external_node_ref::<Material>())?;
            }

            let _skel = r.u32()?;
            r.list(|r| r.f32())?;
            let _vis_cst_type = r.u32()?;

            if r.bool()? {
                let version = r.u32()?;

                if version != 1 {
                    return Err(Error::version("version", version));
                }

                r.u32()?;
                r.f32()?;
                r.bool()?;
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
                r.list(|r| r.box3d())?;
                let _uv_groups = r.list(|r| {
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;

                    Ok(())
                })?;
            }

            let _file_write_time = r.u64()?;
            r.string()?;
            let _materials_folder_name = r.string()?;
            r.string()?;
            let _lights = r.list(|r| {
                r.id()?;

                if r.bool()? {
                    r.external_node_ref::<Light>()?;
                } else {
                    r.string()?;
                }

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
                r.f32()?;
                r.f32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;

                if r.bool()? {
                    r.f32()?;
                    r.f32()?;
                    r.f32()?;
                }

                Ok(())
            })?;
            let _light_user_models: Vec<()> = r.list(|r| todo!())?;
            let _light_insts: Vec<()> = r.list(|r| todo!())?;
            let _damage_zone = r.u32()?;
            let _flags = r.u32()?;
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

            Ok(())
        }

        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
