mod read;

use std::ops::{Deref, DerefMut};

use serde_json_lenient::value::Index;

use crate::{
    common::{Class, ClassId, EngineId},
    Vec2, Vec3,
};

/// Triangles.
#[derive(Default, Debug)]
pub struct VisualIndexedTriangles {
    parent: VisualIndexed,
}

impl Class for VisualIndexedTriangles {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 30);
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

/// Visual geometry that is indexed.
#[derive(Default, Debug)]
pub struct VisualIndexed {
    parent: Visual3D,
    index_buffer: IndexBuffer,
}

impl VisualIndexed {
    pub fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
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

/// 3D visual.
#[derive(Default, Debug)]
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

/// Visual geometry.
#[derive(Default, Debug)]
pub struct Visual {
    /// Vertex data.
    vertex_stream: VertexStream,
}

impl Visual {
    pub fn vertex_stream(&self) -> &VertexStream {
        &self.vertex_stream
    }
}

/// Vertex data.
#[derive(Default, Clone, Debug)]
pub struct VertexStream {
    positions: Vec<Vec3<f32>>,
    texcoords: Vec<Vec2<f32>>,
}

impl VertexStream {
    pub fn positions(&self) -> &[Vec3<f32>] {
        &self.positions
    }
}

impl Class for VertexStream {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 86);
}

#[derive(Default, Debug)]
pub struct IndexBuffer {
    indices: Indices,
}

impl IndexBuffer {
    pub fn indices(&self) -> &Indices {
        &self.indices
    }
}

impl Class for IndexBuffer {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 87);
}

/// Indices.
#[derive(Clone, Debug)]
pub enum Indices {
    /// Indices represented as 16-bit unsigned numbers.
    U16(Vec<u16>),
}

impl Default for Indices {
    fn default() -> Self {
        Self::U16(Vec::default())
    }
}
