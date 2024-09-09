#[test]
fn read_map() {
    gamebox::read_file::<gamebox::Challenge>("tests/engines/game/challenge/Deep_Dip_2r1.Map.Gbx")
        .unwrap();
}
