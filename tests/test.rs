use gamebox::Challenge;

#[test]
fn read_map_alive() {
    let _map: Challenge = gamebox::read_file("tests/files/map/Alive.Map.Gbx").unwrap();
}
