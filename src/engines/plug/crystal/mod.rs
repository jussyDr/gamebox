//! Types used for reading [Crystal] nodes.

mod read;
mod write;

use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{
    common::{Class, ClassId, EngineId},
    Vec3,
};

use super::material_user_inst::MaterialUserInst;

/// A crystal.
#[derive(Default, Debug)]
pub struct Crystal {
    parent: TreeGenerator,
    materials: Vec<Material>,
    layers: Vec<Layer>,
}

impl Crystal {
    pub fn materials(&self) -> &[Material] {
        &self.materials
    }

    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }
}

impl Deref for Crystal {
    type Target = TreeGenerator;

    fn deref(&self) -> &TreeGenerator {
        &self.parent
    }
}

impl DerefMut for Crystal {
    fn deref_mut(&mut self) -> &mut TreeGenerator {
        &mut self.parent
    }
}

impl Class for Crystal {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 3);
}

/// A tree generator.
#[derive(Default, Debug)]
pub struct TreeGenerator;

/// A crystal material
#[derive(Debug)]
pub enum Material {
    Game,
    Custom(Rc<MaterialUserInst>),
}

/// A crystal layer.
#[derive(Debug)]
pub struct Layer {
    id: Rc<str>,
    name: String,
    kind: LayerKind,
}

impl Layer {
    pub fn kind(&self) -> &LayerKind {
        &self.kind
    }
}

#[derive(Debug)]
pub enum LayerKind {
    Geometry {
        mesh: Mesh,
        is_visible: bool,
        is_collidable: bool,
    },
    Smooth,
    Translation,
    Rotation,
    Scale {
        scale: Vec3<f32>,
    },
    Mirror,
    Subdivide,
    Chaos,
    Deformation,
    Cubes,
    Trigger(Mesh),
    SpawnPosition {
        position: Vec3<f32>,
    },
    Lights,
}

/// A mesh.
#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vec3<f32>>,
}

impl Mesh {
    pub fn vertices(&self) -> &[Vec3<f32>] {
        &self.vertices
    }
}
