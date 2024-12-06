//! Crystal.

use std::sync::Arc;

use crate::{Class, Texcoord, Vec3};

use super::{material_user_inst::MaterialUserInst, tree_generator::TreeGenerator};

/// Crystal.
#[derive(PartialEq, Default, Debug)]
pub struct Crystal {
    parent: TreeGenerator,
    materials: Vec<Arc<MaterialUserInst>>,
    geometry_layer: Mesh,
}

impl Class for Crystal {
    const CLASS_ID: u32 = 0x09003000;
}

impl Crystal {
    /// Materials of the crystal.
    pub const fn materials(&self) -> &Vec<Arc<MaterialUserInst>> {
        &self.materials
    }

    /// Geometry of the crystal.
    pub const fn geometry_layer(&self) -> &Mesh {
        &self.geometry_layer
    }
}

/// Mesh of a crystal.
#[derive(PartialEq, Default, Debug)]
pub struct Mesh {
    positions: Vec<Vec3<f32>>,
    faces: Vec<Face>,
}

impl Mesh {
    /// Position data.
    pub const fn positions(&self) -> &Vec<Vec3<f32>> {
        &self.positions
    }

    /// Faces of the mesh.
    pub const fn faces(&self) -> &Vec<Face> {
        &self.faces
    }
}

/// Face of a crystal mesh.
#[derive(PartialEq, Debug)]
pub struct Face {
    indices: Vec<u32>,
    texcoords: Vec<Texcoord>,
    material_index: u32,
    group_index: u32,
}

impl Face {
    pub const fn indices(&self) -> &Vec<u32> {
        &self.indices
    }

    pub const fn texcoords(&self) -> &Vec<Texcoord> {
        &self.texcoords
    }

    pub const fn material_index(&self) -> u32 {
        self.material_index
    }

    pub const fn group_index(&self) -> u32 {
        self.group_index
    }
}

mod read {
    use std::io::{Read, Seek};

    use crate::{
        plug::material_user_inst::MaterialUserInst,
        read::{
            read_body_chunks,
            reader::{IdStateMut, NodeStateMut, Reader},
            BodyChunk, BodyChunks, Error, ErrorKind, ReadBody,
        },
        Texcoord,
    };

    use super::{Crystal, Face, Mesh};

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

        fn read_chunk_5<N>(
            &mut self,
            r: &mut Reader<impl Read, impl IdStateMut, N>,
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

                        let mesh = read_mesh(r)?;
                        r.list(|r| r.u32())?;
                        let _is_visible = r.bool()?;
                        let _is_collidable = r.bool()?;

                        self.geometry_layer = mesh;
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

                        read_mesh(r)?;
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
                    }
                    _ => return Err(Error::new(ErrorKind::Unsupported("layer type".to_string()))),
                }
            }

            Ok(())
        }

        fn read_chunk_6<I, N>(&mut self, r: &mut Reader<impl Read, I, N>) -> Result<(), Error> {
            let version = r.u32()?;

            if version != 1 {
                return Err(Error::chunk_version(version));
            }

            r.list(|r| r.u32())?;

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

    fn read_mesh<I, N>(r: &mut Reader<impl Read, I, N>) -> Result<Mesh, Error> {
        let crystal_version = r.u32()?;

        if crystal_version != 32 {
            return Err(Error::version("crystal", crystal_version));
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
        let _groups = r.list(|r| {
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.string()?;
            r.u32()?;
            r.list(|r| r.u32())?;

            Ok(())
        })?;
        let _is_embedded_crystal = r.bool()?;
        let positions = r.list(|r| r.vec3())?;
        let _edges = r.list(|r| {
            r.u32()?;
            r.u32()?;

            Ok(())
        })?;
        let faces = r.list(|r| {
            let indices = r.list(|r| r.u32())?;
            let texcoords = r.repeat(indices.len(), |r| {
                let u = r.f32()?;
                let v = r.f32()?;

                Ok(Texcoord { u, v })
            })?;
            let material_index = r.u32()?;
            let group_index = r.u32()?;

            Ok(Face {
                indices,
                texcoords,
                material_index,
                group_index,
            })
        })?;
        r.u32()?;
        let num_faces = r.u32()? as usize;
        let num_edges = r.u32()? as usize;
        let num_vertices = r.u32()? as usize;
        r.repeat(num_faces, |r| r.u32())?;
        r.repeat(num_edges, |r| r.u32())?;
        r.repeat(num_vertices, |r| r.u32())?;
        r.u32()?;

        Ok(Mesh { positions, faces })
    }
}
