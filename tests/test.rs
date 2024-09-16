#[test]
fn read_challenge() {
    gamebox::read_file::<gamebox::Challenge>("tests/engines/game/challenge/Deep_Dip_2r1.Map.Gbx")
        .unwrap();
}
