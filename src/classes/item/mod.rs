//! Types used for reading and writing [Item] nodes.

mod read;
mod write;

use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use crate::class::Class;

use super::collector::Collector;

/// Node type corresponding to GameBox files with the extension `Item.Gbx`.
#[derive(Default)]
pub struct Item {
    parent: Collector,
    layers: Vec<(Mesh, ItemMaterial)>,
}

impl Item {
    pub fn layers(&self) -> &[(Mesh, ItemMaterial)] {
        &self.layers
    }
}

#[derive(Clone)]
pub struct Mesh {
    positions: Vec<[f32; 3]>,
    indices: Indices,
}

#[derive(Clone)]
pub enum Indices {
    U16(Vec<u16>),
}

impl Default for Indices {
    fn default() -> Self {
        Self::U16(Vec::default())
    }
}

impl Mesh {
    pub fn positions(&self) -> &[[f32; 3]] {
        &self.positions
    }

    pub fn indices(&self) -> &Indices {
        &self.indices
    }
}

#[derive(Clone)]
pub enum ItemMaterial {
    Game { material_ref: PathBuf },
    Custom(ItemMaterialCustom),
}

#[derive(Clone)]
pub struct ItemMaterialCustom {
    id: String,
    color: [u8; 3],
}

impl Default for ItemMaterial {
    fn default() -> Self {
        Self::Game {
            material_ref: PathBuf::default(),
        }
    }
}

impl Class for Item {
    const ENGINE: u8 = 0x2e;
    const CLASS: u16 = 0x002;
}

impl Deref for Item {
    type Target = Collector;

    fn deref(&self) -> &Collector {
        &self.parent
    }
}

impl DerefMut for Item {
    fn deref_mut(&mut self) -> &mut Collector {
        &mut self.parent
    }
}

#[derive(Default, Clone)]
struct ItemEntityModel {
    solid_to_model: Solid2Model,
}

impl Class for ItemEntityModel {
    const ENGINE: u8 = 0x2e;
    const CLASS: u16 = 0x027;
}

#[derive(Default, Clone)]
struct Solid2Model {
    layers: Vec<(Mesh, ItemMaterial)>,
}

impl Class for Solid2Model {
    const ENGINE: u8 = 0x09;
    const CLASS: u16 = 0x0bb;
}

#[derive(Default, Clone)]
struct MaterialUserInst {
    material: ItemMaterial,
}

impl Class for MaterialUserInst {
    const ENGINE: u8 = 0x09;
    const CLASS: u16 = 0x0fd;
}

#[derive(Default)]
struct VisualIndexedTriangles {
    parent: VisualIndexed,
}

impl Class for VisualIndexedTriangles {
    const ENGINE: u8 = 0x09;
    const CLASS: u16 = 0x01e;
}

impl Deref for VisualIndexedTriangles {
    type Target = VisualIndexed;

    fn deref(&self) -> &VisualIndexed {
        &self.parent
    }
}

impl DerefMut for VisualIndexedTriangles {
    fn deref_mut(&mut self) -> &mut VisualIndexed {
        &mut self.parent
    }
}

#[derive(Default)]
struct VisualIndexed {
    parent: Visual3D,
    indices: Indices,
}

impl Deref for VisualIndexed {
    type Target = Visual3D;

    fn deref(&self) -> &Visual3D {
        &self.parent
    }
}

impl DerefMut for VisualIndexed {
    fn deref_mut(&mut self) -> &mut Visual3D {
        &mut self.parent
    }
}

#[derive(Default)]
struct Visual3D {
    parent: Visual,
}

impl Deref for Visual3D {
    type Target = Visual;

    fn deref(&self) -> &Visual {
        &self.parent
    }
}

impl DerefMut for Visual3D {
    fn deref_mut(&mut self) -> &mut Visual {
        &mut self.parent
    }
}

#[derive(Default)]
struct Visual {
    vertices: VertexStream,
}

#[derive(Default, Clone)]
struct VertexStream {
    positions: Vec<[f32; 3]>,
    texcoords: Vec<()>,
}

impl Class for VertexStream {
    const ENGINE: u8 = 0x09;
    const CLASS: u16 = 0x056;
}

#[derive(Default)]
struct IndexBuffer {
    indices: Indices,
}

impl Class for IndexBuffer {
    const ENGINE: u8 = 0x09;
    const CLASS: u16 = 0x057;
}
