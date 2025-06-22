use std::sync::Arc;

use crate::{Class, ExternalNodeRef, class::visual_indexed_triangles::VisualIndexedTriangles};

#[derive(Default)]
pub struct Solid2Model {
    shaded_geoms: Vec<ShadedGeom>,
    visuals: Vec<Arc<VisualIndexedTriangles>>,
    materials: Vec<ExternalNodeRef>,
    lights: Vec<Light>,
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

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }
}

impl Class for Solid2Model {
    fn class_id(&self) -> u32 {
        0x090BB000
    }
}

pub struct ShadedGeom {
    visual_index: u32,
    material_index: u32,
}

pub struct Light {}

mod read {
    use std::io::Read;

    use crate::{
        class::solid_2_model::{Light, ShadedGeom, Solid2Model},
        read::{
            BodyChunk, BodyChunks, Error, ReadBody, Readable, read_body_chunks,
            reader::{IdsMut, NodesMut, Reader},
        },
    };

    impl Readable for Solid2Model {}

    impl ReadBody for Solid2Model {
        fn read_body(
            &mut self,
            r: &mut Reader<impl Read, impl IdsMut, impl NodesMut>,
        ) -> Result<(), Error> {
            read_body_chunks(r, self)
        }
    }

    impl BodyChunks for Solid2Model {
        type Parent = Self;

        fn parent(&mut self) -> Option<&mut Self::Parent> {
            None
        }

        fn body_chunks<R: Read, I: IdsMut, N: NodesMut>()
        -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::new(0x090bb000, Self::read_chunk_0),
                BodyChunk::skippable(0x090bb002, Self::read_chunk_2),
            ]
            .into_iter()
        }
    }

    impl Solid2Model {
        fn read_chunk_0(
            &mut self,
            r: &mut Reader<impl Read, impl IdsMut, impl NodesMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 34 {
                return Err(Error("unknown chunk version".into()));
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
                self.materials = r.list_with_version(|r| r.external_node_ref())?;
            }

            let skel = r.u32()?;

            if version >= 1 {
                r.list(|r| r.f32())?;
            }

            if version >= 2 {
                let vis_cst_type = r.u32()?;
            }

            if version >= 3 {
                if r.bool32()? {
                    let version = r.u32()?;

                    if version != 1 {
                        return Err(Error("unknown pre light generator version".into()));
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
            }

            if version >= 4 {
                let file_write_time = r.u64()?;
            }

            if version >= 5 {
                r.string()?;
            }

            if version >= 7 {
                let materials_folder_name = r.string()?;
            }

            if version >= 19 {
                r.string()?;
            }

            if version >= 8 {
                self.lights = r.list(|r| {
                    r.id()?;

                    if r.bool32()? {
                        r.external_node_ref()?;
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

                    Ok(Light {})
                })?;
            }

            if version >= 10 {
                let light_user_models: Vec<()> = r.list(|r| todo!())?;
                let light_insts: Vec<()> = r.list(|r| todo!())?;
            }

            if version >= 11 {
                let damage_zone = r.u32()?;
            }

            if version >= 12 {
                let flags = r.u32()?;
            }

            if version >= 13 {
                r.u32()?;
            }

            if version >= 14 {
                r.string()?;
            }

            if version >= 30 {
                r.u32()?;
            }

            if version >= 15 {
                let custom_materials: Vec<()> = r.list(|r| todo!())?;
            }

            if version >= 20 {
                r.list(|r| r.id())?;
            }

            if version >= 22 {
                r.list(|r| r.u32())?;
            }

            if version >= 23 {
                r.u32()?;
            }

            if version >= 24 {
                r.u32()?;
            }

            if version >= 25 {
                r.u32()?;
                r.f32()?;
                r.f32()?;
            }

            if version >= 27 {
                r.id_or_null()?;
            }

            if version >= 31 {
                r.u32()?;
            }

            if version >= 33 {
                r.list(|r| r.u32())?;
            }

            Ok(())
        }

        fn read_chunk_2<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            r.u32()?;
            r.u32()?;

            Ok(())
        }
    }
}
