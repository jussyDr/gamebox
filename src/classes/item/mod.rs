//! Types for `Item`.

mod read;
mod write;

use std::ops::{Deref, DerefMut};

use crate::class::Class;

use super::collector::Collector;

/// Type corresponding to the file extension `Item.Gbx`.
#[derive(Default)]
pub struct Item {
    parent: Collector,
    entity_model: Option<ItemEntityModel>,
}

impl Class for Item {
    const CLASS_ID: u32 = 0x2e002000;
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

#[derive(Default)]
struct ItemEntityModel {
    solid_to_model: Option<Solid2Model>,
}

impl Class for ItemEntityModel {
    const CLASS_ID: u32 = 0x2e027000;
}

#[derive(Default)]
struct Solid2Model {
    meshes: Vec<VisualIndexedTriangles>,
    materials: Vec<MaterialUserInst>,
}

impl Class for Solid2Model {
    const CLASS_ID: u32 = 0x090bb000;
}

#[derive(Default)]
struct MaterialUserInst;

impl Class for MaterialUserInst {
    const CLASS_ID: u32 = 0x090fd000;
}

#[derive(Default)]
struct VisualIndexedTriangles {
    parent: VisualIndexed,
}

impl Class for VisualIndexedTriangles {
    const CLASS_ID: u32 = 0x0901e000;
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
    indices: IndexBuffer,
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

#[derive(Default)]
struct VertexStream;

impl Class for VertexStream {
    const CLASS_ID: u32 = 0x09056000;
}

#[derive(Default)]
struct IndexBuffer;

impl Class for IndexBuffer {
    const CLASS_ID: u32 = 0x09057000;
}
