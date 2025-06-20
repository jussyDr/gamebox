use gamebox::{
    class::{prefab::Prefab, solid2_model::Solid2Model, static_object_model::StaticObjectModel},
    read::read_file,
};

#[test]
fn read_mesh_stade1536v2() {
    let _mesh: Solid2Model = read_file("tests/files/Stade1536v2.Mesh.Gbx").unwrap();
}

#[test]
fn read_prefab_stade1536v2() {
    let _prefab: Prefab = read_file("tests/files/Stade1536v2.Prefab.Gbx").unwrap();
}

#[test]
fn read_static_object_grass4096() {
    let _static_object: StaticObjectModel =
        read_file("tests/files/Grass4096.StaticObject.Gbx").unwrap();
}
