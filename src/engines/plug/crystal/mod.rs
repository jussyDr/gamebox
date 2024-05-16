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
#[derive(Default)]
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
#[derive(Default)]
pub struct TreeGenerator;

struct Layer {
    id: Rc<str>,
    name: String,
    kind: LayerKind,
}

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

struct Mesh {
    vertices: Vec<Vec3<f32>>,
}
