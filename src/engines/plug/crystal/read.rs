use std::io::Read;

use crate::{
    common::Vec2,
    engines::{
        plug::{light_user_model::LightUserModel, material_user_inst::MaterialUserInst},
        script::traits_metadata::TraitsMetadata,
    },
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
    read::{IdStateMut, NodeStateMut, Reader},
    Vec3,
};

use super::{Crystal, Layer, LayerKind, Material, Mesh, TreeGenerator};

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for Crystal {
    fn read_body(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        read_body_chunks(self, r)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for Crystal {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x09051000,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, r| {
                    TreeGenerator::read_chunk_0(&mut n.parent, r)
                }),
            },
            BodyChunkEntry {
                id: 0x09003003,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09003003(n, r)),
            },
            BodyChunkEntry {
                id: 0x09003004,
                read_fn: BodyChunkReadFn::Skippable(|n, r| Self::read_chunk_09003004(n, r)),
            },
            BodyChunkEntry {
                id: 0x09003005,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09003005(n, r)),
            },
            BodyChunkEntry {
                id: 0x09003006,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09003006(n, r)),
            },
            BodyChunkEntry {
                id: 0x09003007,
                read_fn: BodyChunkReadFn::Normal(|n, r| Self::read_chunk_09003007(n, r)),
            },
        ]
        .into_iter()
    }
}

impl Crystal {
    fn read_chunk_09003003<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        let version = r.u32()?;

        if version != 2 {
            return Err("".into());
        }

        self.materials = r.list(|r| {
            let path = r.string()?;

            let material = if path.is_empty() {
                let material = r.internal_node_ref::<MaterialUserInst>()?;

                Material::Custom(material)
            } else {
                Material::Game
            };

            Ok(material)
        })?;

        Ok(())
    }

    fn read_chunk_09003004<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 1
        let size = r.u32()?;

        if size > 0 {
            let mut d = r.take_with(size as u64, (), ());

            r.u32()?; // 0
            r.u32()?; // 1
            r.u32()?; // 0
            r.u32()?; // 1
            r.u32()?; // 1
            r.string()?; // "Layer1"
            r.u32()?; // 0
            r.u32()?; // 1
            r.u32()?; // 0
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?;
            r.u32()?; // 0
            r.node::<TraitsMetadata>()?;
            r.u32()?; // 0
        }

        r.u32()?; // 1

        Ok(())
    }

    fn read_chunk_09003005<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        r: &mut Reader<R, I, N>,
    ) -> Result<()> {
        r.u32()?; // 0
        self.layers = r.list(|r| {
            let kind = r.u32()?;
            r.u32()?; // 2
            r.u32()?; // 0
            let layer_id = r.id()?; // "Layer0"
            let layer_name = r.string()?; // "Geometry"
            r.u32()?; // 1
            let version = r.u32()?;
            let layer_kind = match kind {
                0 => {
                    if version != 1 {
                        return Err("".into());
                    }

                    let mesh = read_mesh(r, self.materials.len() as u32)?;
                    let is_visible = r.bool32()?;
                    let is_collidable = r.bool32()?;

                    LayerKind::Geometry {
                        mesh,
                        is_visible,
                        is_collidable,
                    }
                }
                13 => {
                    if !matches!(version, 7 | 8) {
                        return Err("".into());
                    }

                    r.u32()?; // 64
                    r.u32()?; // 64
                    r.u32()?; // 64
                    r.f32()?;
                    r.u32()?; // 0
                    r.u32()?; // 0
                    r.u32()?; // 0
                    r.u32()?; // 0
                    if version >= 8 {
                        r.f32()?;
                        r.u32()?; // 0
                        r.u32()?; // 0
                        r.u32()?; // 0
                        r.f32()?;
                        r.u32()?; // 0
                        r.u32()?; // 0
                        r.u32()?; // 0
                        r.f32()?;
                        r.u32()?; // 0
                        r.u32()?; // 0
                        r.u32()?; // 0
                    }
                    r.list(|r| {
                        r.u32()?; // 0
                        r.list(|r| {
                            r.u32()?;
                            r.u32()?; // 0
                            r.u32()?; // 0

                            Ok(())
                        })?;

                        Ok(())
                    })?;
                    r.u32()?; // 1
                    r.u32()?; // 1
                    r.list(|r| {
                        r.u32()?;

                        Ok(())
                    })?;

                    LayerKind::Cubes
                }
                14 => {
                    if version != 1 {
                        return Err("".into());
                    }

                    let mesh = read_mesh(r, self.materials.len() as u32)?;

                    LayerKind::Trigger(mesh)
                }

                _ => read_mask_layer(r, kind)?,
            };

            Ok(Layer {
                id: layer_id,
                name: layer_name,
                kind: layer_kind,
            })
        })?;

        Ok(())
    }

    fn read_chunk_09003006<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        let version = r.u32()?;

        if !matches!(version, 0..=2) {
            return Err("".into());
        }

        if version == 0 {
            r.list(|r| {
                r.f32()?;
                r.f32()?;

                Ok(())
            })?;
        } else {
            r.list(|r| {
                r.u32()?;

                Ok(())
            })?;
        }
        if version >= 2 {
            let len = r.u32()?;
            r.repeat(len as usize, |r| {
                read_compact_index(r, len)?;

                Ok(())
            })?;
        }

        Ok(())
    }

    fn read_chunk_09003007<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 0
        r.list(|r| {
            r.f32()?;

            Ok(())
        })?;
        r.list(|r| {
            r.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

impl TreeGenerator {
    fn read_chunk_0<R: Read, I, N>(&mut self, r: &mut Reader<R, I, N>) -> Result<()> {
        r.u32()?; // 1

        Ok(())
    }
}

fn read_mask_layer<R: Read, I: IdStateMut, N: NodeStateMut>(
    r: &mut Reader<R, I, N>,
    kind: u32,
) -> Result<LayerKind> {
    let _mask = r.list(|r| {
        r.u32()?;
        r.id_or_null()?;

        Ok(())
    })?;
    let version = r.u32()?;

    let layer = match kind {
        1 => {
            if version != 0 {
                return Err("".into());
            }

            let _intensity = r.u32()?;

            LayerKind::Smooth
        }
        2 => {
            if version != 0 {
                return Err("".into());
            }

            r.u32()?; // 0
            r.u32()?; // 0
            r.f32()?;

            LayerKind::Translation
        }
        3 => {
            if version != 0 {
                return Err("".into());
            }

            let _rotation = r.f32()?;
            let _axis = r.u32()?;
            let _independent = r.u32()?;

            LayerKind::Rotation
        }
        4 => {
            if version != 0 {
                return Err("".into());
            }

            let scale_x = r.f32()?;
            let scale_y = r.f32()?;
            let scale_z = r.f32()?;
            let _independent = r.u32()?;

            LayerKind::Scale {
                scale: Vec3 {
                    x: scale_x,
                    y: scale_y,
                    z: scale_z,
                },
            }
        }
        5 => {
            if version != 0 {
                return Err("".into());
            }

            let _axis = r.u32()?;
            let _distance = r.f32()?;
            let _independent = r.u32()?;

            LayerKind::Mirror
        }
        8 => {
            if version != 0 {
                return Err("".into());
            }

            let _num_subdivisions = r.u32()?;

            LayerKind::Subdivide
        }
        9 => {
            if version != 1 {
                return Err("".into());
            }

            let _min_distance = r.f32()?;
            r.f32()?;
            let _max_distance = r.f32()?;

            LayerKind::Chaos
        }
        12 => {
            if version != 0 {
                return Err("".into());
            }

            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.u32()?;
            r.f32()?;
            r.u32()?;
            r.u32()?;
            r.f32()?;
            r.f32()?;
            r.f32()?;
            r.list(|r| {
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

                Ok(())
            })?;

            LayerKind::Deformation
        }

        15 => {
            if version != 1 {
                return Err("".into());
            }

            let x = r.f32()?;
            let y = r.f32()?;
            let z = r.f32()?;
            let _yaw = r.f32()?;
            let _pitch = r.f32()?;
            let _roll = r.f32()?;

            LayerKind::SpawnPosition {
                position: Vec3 { x, y, z },
            }
        }
        18 => {
            if version != 0 {
                return Err("".into());
            }

            r.list(|r| {
                r.internal_node_ref::<LightUserModel>()?;

                Ok(())
            })?;
            r.list(|r| {
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
            })?;

            LayerKind::Lights
        }
        _ => return Err("".into()),
    };

    Ok(layer)
}

fn read_mesh<R: Read, I, N>(r: &mut Reader<R, I, N>, num_materials: u32) -> Result<Mesh> {
    let version = r.u32()?;

    if !matches!(version, 32 | 37) {
        return Err("".into());
    }

    r.u32()?; // 4
    r.u32()?; // 3
    r.u32()?; // 4
    r.f32()?; // 64.0
    r.u32()?; // 2
    r.f32()?; // 128.0
    r.u32()?; // 1
    r.f32()?; // 192.0
    r.u32()?; // 0
    let groups = r.list(|r| {
        r.u32()?; // 0
        if version >= 36 {
            r.u8()?; // 1
        } else {
            r.u32()?; // 1
        }
        r.u32()?; // 2 | 0xffffffff
        r.string()?; // "" | "part"
        r.u32()?; // 0xffffffff
        r.list(|r| {
            r.u32()?;

            Ok(())
        })?;

        Ok(())
    })?;
    if version >= 34 {
        r.u8()?; // 1
    } else {
        r.u32()?; // 1
    }
    if version >= 33 {
        r.u32()?; // 1
        r.u32()?;
    }
    let vertices = r.list(|r| {
        let x = r.f32()?;
        let y = r.f32()?;
        let z = r.f32()?;

        Ok(Vec3 { x, y, z })
    })?;
    let num_edges = r.u32()?;
    let _edges = if version >= 35 {
        r.list(|r| {
            read_compact_index(r, vertices.len() as u32)?;
            read_compact_index(r, vertices.len() as u32)?;

            Ok(())
        })?
    } else {
        r.repeat(num_edges as usize, |r| {
            r.u32()?;
            r.u32()?;

            Ok(())
        })?
    };
    let num_faces = r.u32()?;
    if version >= 37 {
        let _texcoords = r.list(|r| {
            let u = r.f32()?;
            let v = r.f32()?;

            Ok(Vec2 { x: u, y: v })
        })?;
        let num_face_indices = r.u32()?;
        let _face_indices = r.repeat(num_face_indices as usize, |r| {
            read_compact_index(r, num_face_indices)?;

            Ok(())
        })?;
    }
    r.repeat(num_faces as usize, |r| {
        let index_count = if version >= 35 {
            r.u8()? + 3
        } else {
            r.u32()? as u8
        };
        let _indices = r.repeat(index_count as usize, |r| {
            let index = if version >= 34 {
                read_compact_index(r, vertices.len() as u32)?
            } else {
                r.u32()?
            };

            Ok(index)
        })?;
        if version <= 36 {
            let _texcoords = r.repeat(index_count as usize, |r| {
                let u = r.f32()?;
                let v = r.f32()?;

                Ok(Vec2 { x: u, y: v })
            })?;
        }
        let _material_index = if version >= 33 {
            read_compact_index(r, num_materials)?
        } else {
            r.u32()?
        };
        let _group_index = if version >= 33 {
            read_compact_index(r, groups.len() as u32)?
        } else {
            r.u32()?
        };

        Ok(())
    })?;
    r.u32()?; // 0
    if version <= 35 {
        let num_faces = r.u32()?;
        let num_edges = r.u32()?;
        let num_vertices = r.u32()?;
        r.repeat(num_faces as usize, |r| {
            r.u32()?;

            Ok(())
        })?;
        r.repeat(num_edges as usize, |r| {
            r.u32()?;

            Ok(())
        })?;
        r.repeat(num_vertices as usize, |r| {
            r.u32()?;

            Ok(())
        })?;
        r.u32()?; // 0
    }
    r.list(|r| {
        r.u32()?;

        Ok(())
    })?;

    Ok(Mesh { vertices })
}

fn read_compact_index<R: Read, I, N>(r: &mut Reader<R, I, N>, len: u32) -> Result<u32> {
    if len < u8::MAX as u32 {
        let index = r.u8()?;
        Ok(index as u32)
    } else if len < u16::MAX as u32 {
        let index = r.u16()?;
        Ok(index as u32)
    } else {
        r.u32()
    }
}
