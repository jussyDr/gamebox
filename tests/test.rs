use gamebox::{Challenge, read_file};

#[test]
fn read_map_alive() {
    let _map: Challenge = read_file("tests/files/map/Alive.Map.Gbx").unwrap();
}
