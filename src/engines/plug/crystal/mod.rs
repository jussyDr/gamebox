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

/// A crystal.
#[derive(Default, Debug)]
pub struct Crystal {
    parent: TreeGenerator,
    materials: Vec<()>,
    layers: Vec<Layer>,
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

#[derive(Debug)]
struct Layer {
    id: Rc<str>,
    name: String,
    kind: LayerKind,
}

#[derive(Debug)]
enum LayerKind {
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

#[derive(Debug)]
struct Mesh {
    vertices: Vec<Vec3<f32>>,
}
