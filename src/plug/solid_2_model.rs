//! Solid 2 model.

use std::sync::Arc;

use crate::{Class, ExternalNodeRef};

use super::{visual_indexed_triangles::VisualIndexedTriangles, Material, MaterialUserInst};

/// Solid 2 model.
#[derive(Default, Debug)]
pub struct Solid2Model {
    shaded_geometries: Vec<ShadedGeometry>,
    materials_folder: Option<String>,
    lights: Vec<()>,
    light_instances: Vec<()>,
}

impl Class for Solid2Model {
    const CLASS_ID: u32 = 0x090bb000;
}

impl Solid2Model {
    /// Shaded geometries.
    pub const fn shaded_geometries(&self) -> &Vec<ShadedGeometry> {
        &self.shaded_geometries
    }

    /// Materials folder.
    pub const fn materials_folder(&self) -> Option<&String> {
        self.materials_folder.as_ref()
    }

    /// Light instances.
    pub const fn light_instances(&self) -> &Vec<()> {
        &self.light_instances
    }
}

/// Shaded geometry.
#[derive(Debug)]
pub struct ShadedGeometry {
    visual: Arc<VisualIndexedTriangles>,
    material: MaterialType,
}

impl ShadedGeometry {
    /// Visual.
    pub const fn visual(&self) -> &Arc<VisualIndexedTriangles> {
        &self.visual
    }

    /// Material.
    pub const fn material(&self) -> &MaterialType {
        &self.material
    }
}

/// Material type.
#[derive(Clone, Debug)]
pub enum MaterialType {
    /// Material.
    Material(ExternalNodeRef<Material>),
    /// User instance.
    UserInst(Arc<MaterialUserInst>),
}

mod read {
    use std::{
        io::{Read, Seek},
        sync::Arc,
    };

    use crate::{
        plug::{
            light::Light, material::Material, visual_indexed_triangles::VisualIndexedTriangles,
            LightUserModel, MaterialUserInst, Skel,
        },
        read::{
            read_body_chunks,
            readable::{HeaderChunk, HeaderChunks, Sealed},
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ReadBody, Readable,
        },
    };

    use super::{MaterialType, ShadedGeometry, Solid2Model};

    impl Readable for Solid2Model {}

    impl Sealed for Solid2Model {}

    impl HeaderChunks for Solid2Model {
        fn header_chunks<R, I, N>() -> impl Iterator<Item = HeaderChunk<Self, R, I, N>> {
            [].into_iter()
        }
    }

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

            if !matches!(version, 30 | 32 | 34) {
                return Err(Error::chunk_version(version));
            }

            r.id_or_null()?;
            let shaded_geometries = r.list(|r| {
                let visual_index = r.u32()?;
                let material_index = r.u32()?;
                r.u32()?;
                let _lod = r.u32()?;

                if version >= 32 {
                    r.u32()?;
                }

                Ok((visual_index, material_index))
            })?;

            let visuals =
                r.list_with_version(|r| r.internal_node_ref::<VisualIndexedTriangles>())?;
            let _material_ids = r.list(|r| r.id())?;
            let material_count = r.u32()?;
            let mut materials = vec![];

            if material_count == 0 {
                materials = r.list_with_version(|r| {
                    let material = r.external_node_ref::<Material>()?;

                    Ok(MaterialType::Material(material))
                })?;
            }

            let skel = r.internal_node_ref_or_null::<Skel>()?;
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
            self.materials_folder = r.string_or_empty()?;
            r.string()?;
            self.lights = r.list(|r| {
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
            let light_user_models = r.list(|r| r.internal_node_ref::<LightUserModel>())?;
            self.light_instances = r.list(|r| {
                let model_index = r.u32()?;
                let socket_index = r.u32()?; // skel

                let _model = light_user_models.get(model_index as usize).unwrap();

                let _socket = skel
                    .as_ref()
                    .unwrap()
                    .sockets()
                    .get(socket_index as usize)
                    .unwrap();

                Ok(())
            })?;
            let _damage_zone = r.u32()?;
            let _flags = r.u32()?;
            r.u32()?;
            r.string()?;
            r.u32()?;
            if material_count != 0 {
                materials = r.repeat(material_count as usize, |r| {
                    let _name = r.string()?;
                    let material = r.internal_node_ref::<MaterialUserInst>()?;

                    Ok(MaterialType::UserInst(material))
                })?;
            }
            self.shaded_geometries = shaded_geometries
                .into_iter()
                .map(|(visual_index, material_index)| {
                    let visual = Arc::clone(visuals.get(visual_index as usize).unwrap());
                    let material = materials.get(material_index as usize).unwrap().clone();

                    ShadedGeometry { visual, material }
                })
                .collect();
            r.list(|r| r.id())?;
            r.list(|r| r.u32())?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;

            if version >= 31 {
                r.u32()?;
            }

            if version >= 33 {
                r.u32()?;
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
