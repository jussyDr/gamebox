use std::rc::Rc;

use crate::{
    common::{Class, ClassId, EngineId},
    Vec3,
};

mod read;

#[derive(Default)]
pub struct Crystal {
    parent: TreeGenerator,
    materials: Vec<()>,
    layers: Vec<Layer>,
}

impl Class for Crystal {
    const CLASS_ID: ClassId = ClassId::new(EngineId::PLUG, 3);
}

#[derive(Default)]
struct TreeGenerator;

struct Layer {
    id: Rc<str>,
    name: String,
    kind: LayerKind,
}

enum LayerKind {
    Geometry(Mesh),
    Smooth,
    Translation,
    Rotation,
    Scale,
    Mirror,
    Subdivide,
    Chaos,
    Deformation,
    Cubes,
    Trigger(Mesh),
    Spawnposition,
    Lights,
}

struct Mesh {
    vertices: Vec<Vec3<f32>>,
}
