mod read;

use std::ops::{Deref, DerefMut};

use crate::{class::ClassId, EngineId};

#[derive(Default)]
pub struct VisualIndexedTriangles {
    parent: VisualIndexed,
}

impl ClassId for VisualIndexedTriangles {
    const ENGINE: u8 = EngineId::PLUG;
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
pub struct VisualIndexed {
    parent: Visual3D,
    pub indices: Indices,
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
pub struct Visual3D {
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
pub struct Visual {
    pub vertices: VertexStream,
}

#[derive(Default, Clone)]
pub struct VertexStream {
    pub positions: Vec<[f32; 3]>,
    pub texcoords: Vec<[f32; 2]>,
}

impl ClassId for VertexStream {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x056;
}

#[derive(Default)]
struct IndexBuffer {
    indices: Indices,
}

impl ClassId for IndexBuffer {
    const ENGINE: u8 = EngineId::PLUG;
    const CLASS: u16 = 0x057;
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
