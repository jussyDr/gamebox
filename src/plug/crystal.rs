//! Crystal.

use std::sync::Arc;

use crate::{Class, Vec3};

use super::{material_user_inst::MaterialUserInst, tree_generator::TreeGenerator};

/// Crystal.
#[derive(Default)]
pub struct Crystal {
    parent: TreeGenerator,
    materials: Vec<Arc<MaterialUserInst>>,
    geometry: Mesh,
    trigger: Option<Mesh>,
    spawn_position: Option<SpawnPosition>,
}

impl Class for Crystal {
    const CLASS_ID: u32 = 0x09003000;
}

impl Crystal {
    /// Materials.
    pub const fn materials(&self) -> &Vec<Arc<MaterialUserInst>> {
        &self.materials
    }

    /// Geometry.
    pub const fn geometry(&self) -> &Mesh {
        &self.geometry
    }

    /// Trigger.
    pub const fn trigger(&self) -> Option<&Mesh> {
        self.trigger.as_ref()
    }

    /// Spawn position.
    pub const fn spawn_position(&self) -> Option<&SpawnPosition> {
        self.spawn_position.as_ref()
    }
}

/// Crystal mesh.
#[derive(Default)]
pub struct Mesh {
    positions: Vec<Vec3<f32>>,
    faces: Vec<Face>,
}

impl Mesh {
    /// Positions.
    pub const fn positions(&self) -> &Vec<Vec3<f32>> {
        &self.positions
    }

    /// Faces.
    pub const fn faces(&self) -> &Vec<Face> {
        &self.faces
    }
}

/// Face of a crystal mesh.
pub struct Face {
    indices: Vec<u32>,
    material_index: u32,
    group_index: u32,
}

impl Face {
    /// Indices
    pub const fn indices(&self) -> &Vec<u32> {
        &self.indices
    }

    /// Material index.
    pub const fn material_index(&self) -> u32 {
        self.material_index
    }

    /// Group index.
    pub const fn group_index(&self) -> u32 {
        self.group_index
    }
}

/// Spawn position.
pub struct SpawnPosition;

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::{material_user_inst::MaterialUserInst, LightUserModel},
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ErrorKind, ReadBody,
        },
        Texcoord,
    };

    use super::{Crystal, Face, Mesh, SpawnPosition};

    impl ReadBody for Crystal {
        fn read_body<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
            &mut self,
            r: &mut Reader<R, I, N>,
        ) -> Result<(), Error> {
            read_body_chunks(self, r)
        }
    }

    impl BodyChunks for Crystal {
        fn parent(&mut self) -> Option<&mut impl BodyChunks> {
            Some(&mut self.parent)
        }

        fn body_chunks<R: Read + Seek, I: IdStateMut, N: NodeStateMut>(
        ) -> impl Iterator<Item = BodyChunk<Self, R, I, N>> {
            [
                BodyChunk::normal(3, Self::read_chunk_3),
                BodyChunk::skippable(4, Self::read_chunk_4),
                BodyChunk::normal(5, Self::read_chunk_5),
                BodyChunk::normal(6, Self::read_chunk_6),
                BodyChunk::normal(7, Self::read_chunk_7),
            ]
            .into_iter()
        }
    }

    impl Crystal {
        fn read_chunk_3(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 2 {
                return Err(Error::chunk_version(version));
            }

            self.materials = r.list(|r| {
                let name = r.string()?;

                let material = if name.is_empty() {
                    r.internal_node_ref::<MaterialUserInst>()?
                } else {
                    todo!()
                };

                Ok(material)
            })?;

            Ok(())
        }

        fn read_chunk_4<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            r.byte_buf()?;
            r.u32()?;

            Ok(())
        }

        fn read_chunk_5(
            &mut self,
            r: &mut Reader<impl Read + Seek, impl IdStateMut, impl NodeStateMut>,
        ) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let num_layers = r.u32()?;

            for _ in 0..num_layers {
                let layer_type = r.u32()?;

                match layer_type {
                    0 => {
                        let version = r.u32()?;

                        if version != 2 {
                            return Err(Error::version("layer", version));
                        }

                        let _crystal_enabled = r.bool()?;
                        let _layer_id = r.id()?;
                        let _layer_name = r.string()?;
                        let _is_enabled = r.bool()?;
                        let geometry_version = r.u32()?;

                        if geometry_version != 1 {
                            return Err(Error::version("geometry", geometry_version));
                        }

                        self.geometry = read_mesh(r, self.materials.len() as u32)?;
                        r.list(|r| r.u32())?;
                        let _is_visible = r.bool()?;
                        let _is_collidable = r.bool()?;
                    }
                    14 => {
                        let version = r.u32()?;

                        if version != 2 {
                            return Err(Error::version("layer", version));
                        }

                        let _crystal_enabled = r.bool()?;
                        let _layer_id = r.id()?;
                        let _layer_name = r.string()?;
                        let _is_enabled = r.bool()?;
                        let trigger_version = r.u32()?;

                        if trigger_version != 1 {
                            return Err(Error::version("trigger", trigger_version));
                        }

                        self.trigger = Some(read_mesh(r, self.materials.len() as u32)?);
                        r.list(|r| r.u32())?;
                    }
                    15 => {
                        let version = r.u32()?;

                        if version != 2 {
                            return Err(Error::version("layer", version));
                        }

                        let _crystal_enabled = r.bool()?;
                        let _layer_id = r.id()?;
                        let _layer_name = r.string()?;
                        let _is_enabled = r.bool()?;
                        let modifier_version = r.u32()?;

                        if modifier_version != 0 {
                            return Err(Error::version("modifier", modifier_version));
                        }

                        let _mask = r.list(|r| {
                            let _group_index = r.u32()?;
                            let _layer_id = r.id()?;

                            Ok(())
                        })?;
                        let spawn_position_version = r.u32()?;

                        if spawn_position_version != 1 {
                            return Err(Error::version("spawn position", spawn_position_version));
                        }

                        let _spawn_position = r.vec3::<f32>()?;
                        let _horizontal_angle = r.f32()?;
                        let _vertical_angle = r.f32()?;
                        let _roll_angle = r.f32()?;

                        self.spawn_position = Some(SpawnPosition);
                    }
                    18 => {
                        let version = r.u32()?;

                        if version != 2 {
                            return Err(Error::version("layer", version));
                        }

                        let _crystal_enabled = r.bool()?;
                        let _layer_id = r.id()?;
                        let _layer_name = r.string()?;
                        let _is_enabled = r.bool()?;
                        let modifier_version = r.u32()?;

                        if modifier_version != 0 {
                            return Err(Error::version("modifier", modifier_version));
                        }

                        let _mask = r.list(|r| {
                            let _group_index = r.u32()?;
                            let _layer_id = r.id()?;

                            Ok(())
                        })?;

                        let light_version = r.u32()?;

                        if light_version != 0 {
                            return Err(Error::version("light", light_version));
                        }

                        let _lights = r.list(|r| r.internal_node_ref::<LightUserModel>())?;
                        let _light_positions = r.list(|r| {
                            r.u32()?;
                            r.vec3::<f32>()?;
                            r.vec3::<f32>()?;
                            r.vec3::<f32>()?;
                            r.vec3::<f32>()?;

                            Ok(())
                        })?;
                    }
                    _ => {
                        return Err(Error::new(ErrorKind::Unsupported(format!(
                            "layer type variant: {layer_type}"
                        ))))
                    }
                }
            }

            Ok(())
        }

        fn read_chunk_6<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if !matches!(version, 0..=2) {
                return Err(Error::chunk_version(version));
            }

            if version == 0 {
                r.list(|r| r.vec2::<f32>())?;
            } else {
                r.list(|r| r.u32())?;

                if version >= 2 {
                    let num = r.u32()?;
                    r.repeat(num as usize, |r| read_index(r, num))?;
                }
            }

            Ok(())
        }

        fn read_chunk_7<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 0 {
                return Err(Error::chunk_version(version));
            }

            let _smoothing_groups = r.list(|r| r.f32())?;
            r.list(|r| r.u32())?;

            Ok(())
        }
    }

    fn read_mesh<I, N>(r: &mut Reader<impl Read, I, N>, num_materials: u32) -> Result<Mesh, Error> {
        let mesh_version = r.u32()?;

        if !matches!(mesh_version, 32 | 37) {
            return Err(Error::version("crystal mesh", mesh_version));
        }

        r.u32()?;
        let _visual_levels = r.list(|r| {
            r.u32()?;
            r.f32()?;

            Ok(())
        })?;
        let _anchor_infos = r.list(|r| {
            r.bool()?;
            r.bool()?;
            r.vec3::<f32>()?;
            r.vec3::<f32>()?;
            r.vec3::<f32>()?;
            r.vec3::<f32>()?;
            r.string()?;
            r.u32()?;

            Ok(())
        })?;
        let groups = r.list(|r| {
            r.u32()?;

            if mesh_version >= 36 {
                r.u8()?;
            } else {
                r.u32()?;
            }

            r.u32()?;
            r.string()?;
            r.u32()?;
            r.list(|r| r.u32())?;

            Ok(())
        })?;

        let _is_embedded_crystal = if mesh_version >= 34 {
            r.bool8()?
        } else {
            r.bool()?
        };

        if mesh_version >= 33 {
            r.u32()?;
            r.u32()?;
        }

        let positions = r.list(|r| r.vec3())?;
        let mut num_edges = r.u32()?;

        if mesh_version >= 35 {
            num_edges = r.u32()?;
        }

        let _edges = r.repeat(num_edges as usize, |r| {
            if mesh_version >= 35 {
                read_index(r, num_edges)?;
                read_index(r, num_edges)?;
            } else {
                r.u32()?;
                r.u32()?;
            }

            Ok(())
        })?;
        let num_faces = r.u32()?;

        if mesh_version >= 37 {
            let _texcoords = r.list(|r| r.vec2::<f32>())?;
            let num_texcoord_indices = r.u32()?;
            let _texcoord_indices = r.repeat(num_texcoord_indices as usize, |r| {
                read_index(r, num_texcoord_indices)
            })?;
        }

        let faces = r.repeat(num_faces as usize, |r| {
            let num_verts = if mesh_version >= 35 {
                r.u8()? as u32 + 3
            } else {
                r.u32()?
            };
            let indices = r.repeat(num_verts as usize, |r| {
                if mesh_version >= 34 {
                    read_index(r, positions.len() as u32)
                } else {
                    r.u32()
                }
            })?;
            if mesh_version < 37 {
                let _texcoords = r.repeat(indices.len(), |r| {
                    let u = r.f32()?;
                    let v = r.f32()?;

                    Ok(Texcoord { u, v })
                })?;
            }
            let material_index = if mesh_version >= 33 {
                read_index(r, num_materials)?
            } else {
                r.u32()?
            };
            let group_index = if mesh_version >= 33 {
                read_index(r, groups.len() as u32)?
            } else {
                r.u32()?
            };

            Ok(Face {
                indices,
                material_index,
                group_index,
            })
        })?;
        r.u32()?;

        if mesh_version < 36 {
            let num_faces = r.u32()? as usize;
            let num_edges = r.u32()? as usize;
            let num_vertices = r.u32()? as usize;
            r.repeat(num_faces, |r| r.u32())?;
            r.repeat(num_edges, |r| r.u32())?;
            r.repeat(num_vertices, |r| r.u32())?;
            r.u32()?;
        }

        Ok(Mesh { positions, faces })
    }

    fn read_index<I, N>(r: &mut Reader<impl Read, I, N>, max: u32) -> Result<u32, Error> {
        let value = if max <= u8::MAX as u32 {
            r.u8()? as u32
        } else if max <= u16::MAX as u32 {
            r.u16()? as u32
        } else {
            r.u32()?
        };

        Ok(value)
    }
}
