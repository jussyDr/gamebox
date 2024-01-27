use std::io::Read;

use crate::{
    classes::{
        light_user_model::LightUserModel, material_user_inst::MaterialUserInst,
        traits_metadata::TraitsMetadata,
    },
    common::Vec2,
    deserialize::{Deserializer, IdStateMut, NodeStateMut},
    read::{
        readable::{read_body_chunks, BodyChunkEntry, BodyChunkReadFn, BodyChunks, ReadBody},
        Result,
    },
    Vec3,
};

use super::{Crystal, Layer, LayerKind, Mesh, TreeGenerator};

impl<R: Read, I: IdStateMut, N: NodeStateMut> ReadBody<R, I, N> for Crystal {
    fn read_body(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        read_body_chunks(self, d)
    }
}

impl<R: Read, I: IdStateMut, N: NodeStateMut> BodyChunks<R, I, N> for Crystal {
    #[allow(clippy::redundant_closure)]
    fn body_chunks() -> impl Iterator<Item = BodyChunkEntry<Self, R, I, N>> {
        [
            BodyChunkEntry {
                id: 0x09051000,
                read_fn: BodyChunkReadFn::Normal(|n: &mut Self, d| {
                    TreeGenerator::read_chunk_0(&mut n.parent, d)
                }),
            },
            BodyChunkEntry {
                id: 0x09003003,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09003003(n, d)),
            },
            BodyChunkEntry {
                id: 0x09003004,
                read_fn: BodyChunkReadFn::Skippable(|n, d| Self::read_chunk_09003004(n, d)),
            },
            BodyChunkEntry {
                id: 0x09003005,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09003005(n, d)),
            },
            BodyChunkEntry {
                id: 0x09003006,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09003006(n, d)),
            },
            BodyChunkEntry {
                id: 0x09003007,
                read_fn: BodyChunkReadFn::Normal(|n, d| Self::read_chunk_09003007(n, d)),
            },
        ]
        .into_iter()
    }
}

impl Crystal {
    fn read_chunk_09003003<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        let version = d.u32()?;

        if version != 2 {
            return Err("".into());
        }

        self.materials = d.list(|d| {
            let path = d.string()?;
            if path.is_empty() {
                d.internal_node_ref::<MaterialUserInst>()?;
            }

            Ok(())
        })?;

        Ok(())
    }

    fn read_chunk_09003004<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1
        let size = d.u32()?;

        if size > 0 {
            let mut d = d.take_with(size as u64, (), ());

            d.u32()?; // 0
            d.u32()?; // 1
            d.u32()?; // 0
            d.u32()?; // 1
            d.u32()?; // 1
            d.string()?; // "Layer1"
            d.u32()?; // 0
            d.u32()?; // 1
            d.u32()?; // 0
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?;
            d.u32()?; // 0
            d.node::<TraitsMetadata>()?;
            d.u32()?; // 0
        }

        d.u32()?; // 1

        Ok(())
    }

    fn read_chunk_09003005<R: Read, I: IdStateMut, N: NodeStateMut>(
        &mut self,
        d: &mut Deserializer<R, I, N>,
    ) -> Result<()> {
        d.u32()?; // 0
        self.layers = d.list(|d| {
            let kind = d.u32()?;
            d.u32()?; // 2
            d.u32()?; // 0
            let layer_id = d.id()?; // "Layer0"
            let layer_name = d.string()?; // "Geometry"
            d.u32()?; // 1
            let version = d.u32()?;
            let layer_kind = match kind {
                0 => {
                    let mesh = read_mesh(d, self.materials.len() as u32)?;
                    d.u32()?; // 1
                    d.u32()?; // 0 | 1

                    LayerKind::Geometry(mesh)
                }
                1 => {
                    d.list(|d| {
                        d.u32()?;
                        d.id_or_null()?;

                        Ok(())
                    })?;
                    d.u32()?; // 0
                    d.u32()?; // 1

                    LayerKind::Smooth
                }
                2 => {
                    d.list(|d| {
                        d.u32()?;
                        d.id_or_null()?;

                        Ok(())
                    })?;
                    d.u32()?; // 0
                    d.u32()?; // 0
                    d.u32()?; // 0
                    d.f32()?;

                    LayerKind::Translation
                }
                3 => {
                    d.list(|d| {
                        d.u32()?;
                        d.id_or_null()?;

                        Ok(())
                    })?;
                    d.u32()?; // 0
                    d.f32()?;
                    d.u32()?; // 2
                    d.u32()?; // 0

                    LayerKind::Rotation
                }
                4 => {
                    d.list(|d| {
                        d.u32()?;
                        d.id_or_null()?;

                        Ok(())
                    })?;
                    d.u32()?; // 0
                    d.f32()?; // 32.0
                    d.f32()?; // 32.0
                    d.f32()?; // 32.0
                    d.u32()?; // 0

                    LayerKind::Scale
                }
                5 => {
                    d.list(|d| {
                        d.u32()?;
                        d.id_or_null()?;

                        Ok(())
                    })?;
                    d.u32()?; // 0
                    d.u32()?; // 2
                    d.u32()?; // 0
                    d.u32()?; // 0

                    LayerKind::Mirror
                }
                8 => {
                    d.list(|d| {
                        d.u32()?;
                        d.id_or_null()?;

                        Ok(())
                    })?;
                    d.u32()?; // 0
                    d.u32()?; // 1

                    LayerKind::Subdivide
                }
                9 => {
                    d.list(|d| {
                        d.u32()?;
                        d.id_or_null()?;

                        Ok(())
                    })?;
                    d.u32()?; // 1
                    d.u32()?;
                    d.u32()?;
                    d.u32()?;

                    LayerKind::Chaos
                }
                12 => {
                    d.list(|d| {
                        d.u32()?;
                        d.id_or_null()?;

                        Ok(())
                    })?;
                    d.u32()?; // 0
                    d.f32()?;
                    d.f32()?;
                    d.f32()?;
                    d.f32()?;
                    d.f32()?;
                    d.f32()?;
                    d.u32()?;
                    d.u32()?;
                    d.f32()?;
                    d.f32()?;
                    d.f32()?;
                    d.u32()?;
                    d.f32()?;
                    d.u32()?;
                    d.u32()?;
                    d.f32()?;
                    d.f32()?;
                    d.f32()?;
                    d.list(|d| {
                        d.f32()?;
                        d.f32()?;
                        d.f32()?;
                        d.f32()?;
                        d.f32()?;
                        d.f32()?;
                        d.f32()?;
                        d.f32()?;
                        d.f32()?;
                        d.f32()?;
                        d.f32()?;
                        d.f32()?;

                        Ok(())
                    })?;

                    LayerKind::Deformation
                }
                13 => {
                    if !matches!(version, 7 | 8) {
                        return Err("".into());
                    }

                    d.u32()?; // 64
                    d.u32()?; // 64
                    d.u32()?; // 64
                    d.f32()?;
                    d.u32()?; // 0
                    d.u32()?; // 0
                    d.u32()?; // 0
                    d.u32()?; // 0
                    if version >= 8 {
                        d.f32()?;
                        d.u32()?; // 0
                        d.u32()?; // 0
                        d.u32()?; // 0
                        d.f32()?;
                        d.u32()?; // 0
                        d.u32()?; // 0
                        d.u32()?; // 0
                        d.f32()?;
                        d.u32()?; // 0
                        d.u32()?; // 0
                        d.u32()?; // 0
                    }
                    d.list(|d| {
                        d.u32()?; // 0
                        d.list(|d| {
                            d.u32()?;
                            d.u32()?; // 0
                            d.u32()?; // 0

                            Ok(())
                        })?;

                        Ok(())
                    })?;
                    d.u32()?; // 1
                    d.u32()?; // 1
                    d.list(|d| {
                        d.u32()?;

                        Ok(())
                    })?;

                    LayerKind::Cubes
                }
                14 => {
                    let mesh = read_mesh(d, self.materials.len() as u32)?;

                    LayerKind::Trigger(mesh)
                }
                15 => {
                    d.list(|d| {
                        d.u32()?;
                        d.id_or_null()?;

                        Ok(())
                    })?;
                    d.u32()?; // 1
                    d.f32()?;
                    d.f32()?;
                    d.f32()?;
                    d.f32()?;
                    d.f32()?;
                    d.f32()?;

                    LayerKind::Spawnposition
                }
                18 => {
                    d.list(|d| {
                        d.u32()?;
                        d.id_or_null()?;

                        Ok(())
                    })?;
                    d.u32()?; // 0
                    d.list(|d| {
                        d.internal_node_ref::<LightUserModel>()?;

                        Ok(())
                    })?;
                    d.list(|d| {
                        d.u32()?;
                        d.u32()?;
                        d.u32()?;
                        d.u32()?;
                        d.u32()?;
                        d.u32()?;
                        d.u32()?;
                        d.u32()?;
                        d.u32()?;
                        d.u32()?;
                        d.u32()?;
                        d.u32()?;
                        d.u32()?;

                        Ok(())
                    })?;

                    LayerKind::Lights
                }
                _ => return Err("".into()),
            };

            Ok(Layer {
                id: layer_id,
                name: layer_name,
                kind: layer_kind,
            })
        })?;

        Ok(())
    }

    fn read_chunk_09003006<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        let version = d.u32()?;

        if !matches!(version, 0..=2) {
            return Err("".into());
        }

        if version == 0 {
            d.list(|d| {
                d.f32()?;
                d.f32()?;

                Ok(())
            })?;
        } else {
            d.list(|d| {
                d.u32()?;

                Ok(())
            })?;
        }
        if version >= 2 {
            let len = d.u32()?;
            d.repeat(len as usize, |d| {
                read_compact_index(d, len)?;

                Ok(())
            })?;
        }

        Ok(())
    }

    fn read_chunk_09003007<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 0
        d.list(|d| {
            d.f32()?;

            Ok(())
        })?;
        d.list(|d| {
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    }
}

impl TreeGenerator {
    fn read_chunk_0<R: Read, I, N>(&mut self, d: &mut Deserializer<R, I, N>) -> Result<()> {
        d.u32()?; // 1

        Ok(())
    }
}

fn read_mesh<R: Read, I, N>(d: &mut Deserializer<R, I, N>, num_materials: u32) -> Result<Mesh> {
    let version = d.u32()?;

    if !matches!(version, 32 | 37) {
        return Err("".into());
    }

    d.u32()?; // 4
    d.u32()?; // 3
    d.u32()?; // 4
    d.f32()?; // 64.0
    d.u32()?; // 2
    d.f32()?; // 128.0
    d.u32()?; // 1
    d.f32()?; // 192.0
    d.u32()?; // 0
    let groups = d.list(|d| {
        d.u32()?; // 0
        if version >= 36 {
            d.u8()?; // 1
        } else {
            d.u32()?; // 1
        }
        d.u32()?; // 2 | 0xffffffff
        d.string()?; // "" | "part"
        d.u32()?; // 0xffffffff
        d.list(|d| {
            d.u32()?;

            Ok(())
        })?;

        Ok(())
    })?;
    if version >= 34 {
        d.u8()?; // 1
    } else {
        d.u32()?; // 1
    }
    if version >= 33 {
        d.u32()?; // 1
        d.u32()?;
    }
    let vertices = d.list(|d| {
        let x = d.f32()?;
        let y = d.f32()?;
        let z = d.f32()?;

        Ok(Vec3 { x, y, z })
    })?;
    let num_edges = d.u32()?;
    let _edges = if version >= 35 {
        d.list(|d| {
            read_compact_index(d, vertices.len() as u32)?;
            read_compact_index(d, vertices.len() as u32)?;

            Ok(())
        })?
    } else {
        d.repeat(num_edges as usize, |d| {
            d.u32()?;
            d.u32()?;

            Ok(())
        })?
    };
    let num_faces = d.u32()?;
    if version >= 37 {
        let _texcoords = d.list(|d| {
            let u = d.f32()?;
            let v = d.f32()?;

            Ok(Vec2 { x: u, y: v })
        })?;
        let num_face_indices = d.u32()?;
        let _face_indices = d.repeat(num_face_indices as usize, |d| {
            read_compact_index(d, num_face_indices)?;

            Ok(())
        })?;
    }
    d.repeat(num_faces as usize, |d| {
        let index_count = if version >= 35 {
            d.u8()? + 3
        } else {
            d.u32()? as u8
        };
        let _indices = d.repeat(index_count as usize, |d| {
            let index = if version >= 34 {
                read_compact_index(d, vertices.len() as u32)?
            } else {
                d.u32()?
            };

            Ok(index)
        })?;
        if version <= 36 {
            let _texcoords = d.repeat(index_count as usize, |d| {
                let u = d.f32()?;
                let v = d.f32()?;

                Ok(Vec2 { x: u, y: v })
            })?;
        }
        let _material_index = if version >= 33 {
            read_compact_index(d, num_materials)?
        } else {
            d.u32()?
        };
        let _group_index = if version >= 33 {
            read_compact_index(d, groups.len() as u32)?
        } else {
            d.u32()?
        };

        Ok(())
    })?;
    d.u32()?; // 0
    if version <= 35 {
        let num_faces = d.u32()?;
        let num_edges = d.u32()?;
        let num_vertices = d.u32()?;
        d.repeat(num_faces as usize, |d| {
            d.u32()?;

            Ok(())
        })?;
        d.repeat(num_edges as usize, |d| {
            d.u32()?;

            Ok(())
        })?;
        d.repeat(num_vertices as usize, |d| {
            d.u32()?;

            Ok(())
        })?;
        d.u32()?; // 0
    }
    d.list(|d| {
        d.u32()?;

        Ok(())
    })?;

    Ok(Mesh { vertices })
}

fn read_compact_index<R: Read, I, N>(d: &mut Deserializer<R, I, N>, len: u32) -> Result<u32> {
    if len < u8::MAX as u32 {
        let index = d.u8()?;
        Ok(index as u32)
    } else if len < u16::MAX as u32 {
        let index = d.u16()?;
        Ok(index as u32)
    } else {
        d.u32()
    }
}
