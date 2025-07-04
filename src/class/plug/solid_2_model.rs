//! Solid 2 model.

use std::sync::Arc;

use crate::{
    ClassId, ExternalNodeRef, SubExtensions,
    class::plug::visual_indexed_triangles::VisualIndexedTriangles,
};

/// A 3D model.
#[derive(Default)]
pub struct Solid2Model {
    shaded_geoms: Vec<ShadedGeom>,
    visuals: Vec<Arc<VisualIndexedTriangles>>,
    materials: Vec<ExternalNodeRef>,
    lights: Vec<Solid2ModelLight>,
}

impl Solid2Model {
    pub fn shaded_geoms(&self) -> &Vec<ShadedGeom> {
        &self.shaded_geoms
    }

    pub fn visuals(&self) -> &Vec<Arc<VisualIndexedTriangles>> {
        &self.visuals
    }

    pub fn materials(&self) -> &Vec<ExternalNodeRef> {
        &self.materials
    }

    pub fn lights(&self) -> &Vec<Solid2ModelLight> {
        &self.lights
    }
}

impl ClassId for Solid2Model {
    const CLASS_ID: u32 = 0x090bb000;
}

impl SubExtensions for Solid2Model {
    const SUB_EXTENSIONS: &[&str] = &["Mesh"];
}

pub struct ShadedGeom {
    visual_index: u32,
    material_index: u32,
}

impl ShadedGeom {
    pub fn visual_index(&self) -> u32 {
        self.visual_index
    }

    pub fn material_index(&self) -> u32 {
        self.material_index
    }
}

pub struct Solid2ModelLight {}

mod read {
    use crate::{
        class::plug::{
            light::Light,
            material::Material,
            solid_2_model::{ShadedGeom, Solid2Model, Solid2ModelLight},
        },
        read::{
            BodyChunk, BodyChunks, Error, HeaderChunk, HeaderChunks, ReadBody, Readable,
            error_unknown_chunk_version, error_unknown_version, read_body_chunks,
            reader::{BodyReader, HeaderReader},
        },
    };

    impl Readable for Solid2Model {}

    impl HeaderChunks for Solid2Model {
        fn header_chunks<R: HeaderReader>() -> impl IntoIterator<Item = HeaderChunk<Self, R>> {
            []
        }
    }

    impl ReadBody for Solid2Model {
        fn read_body(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Solid2Model {
        fn body_chunks<R: BodyReader>() -> impl IntoIterator<Item = BodyChunk<Self, R>> {
            [
                BodyChunk::new(0, Self::read_chunk_0),
                BodyChunk::skippable(2, Self::read_chunk_2),
            ]
        }
    }

    impl Solid2Model {
        fn read_chunk_0(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 34 {
                return Err(error_unknown_chunk_version(version));
            }

            let u01 = r.id_or_null()?;
            self.shaded_geoms = r.list(|r| {
                let visual_index = r.u32()?;
                let material_index = r.u32()?;
                r.u32()?;
                let lod = r.u32()?;
                r.u32()?;

                Ok(ShadedGeom {
                    visual_index,
                    material_index,
                })
            })?;
            self.visuals = r.list_with_version(|r| r.internal_node_ref())?;
            let material_ids = r.list(|r| r.id())?;
            let material_count = r.u32()?;
            if material_count == 0 {
                self.materials = r.list_with_version(|r| r.external_node_ref::<Material>())?;
            }
            let skel = r.u32()?;
            r.list(|r| r.f32())?;
            let vis_cst_type = r.u32()?;
            if r.bool32()? {
                let version = r.u32()?;

                if version != 1 {
                    return Err(error_unknown_version("pre light generator", version));
                }

                r.u32()?;
                r.f32()?;
                r.bool32()?;
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
                let uv_groups = r.list(|r| {
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;
                    r.u32()?;

                    Ok(())
                })?;
            }
            let file_write_time = r.u64()?;
            r.string()?;
            let materials_folder_name = r.string()?;
            r.string()?;
            self.lights = r.list(|r| {
                r.id()?;

                if r.bool32()? {
                    r.external_node_ref::<Light>()?;
                } else {
                    todo!()
                }

                r.iso4()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;
                r.u32()?;

                if r.bool32()? {
                    r.f32()?;
                    r.f32()?;
                    r.f32()?;
                }

                Ok(Solid2ModelLight {})
            })?;
            let light_user_models: Vec<()> = r.list(|r| todo!())?;
            let light_insts: Vec<()> = r.list(|r| todo!())?;
            let damage_zone = r.u32()?;
            let flags = r.u32()?;
            r.u32()?;
            r.string()?;
            r.u32()?;
            let custom_materials: Vec<()> = r.list(|r| todo!())?;
            r.list(|r| r.id())?;
            r.list(|r| r.u32())?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.f32()?;
            r.id_or_null()?;
            r.u32()?;
            r.list(|r| r.u32())?;

            Ok(())
        }

        fn read_chunk_2(&mut self, r: &mut impl BodyReader) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
