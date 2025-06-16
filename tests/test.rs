use gamebox::{class::prefab::Prefab, read::read_file};

#[test]
fn read_prefab_stade1536v2() {
    let _prefab: Prefab = read_file("tests/files/Stade1536v2.Prefab.Gbx").unwrap();
}
