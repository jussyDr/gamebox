#[test]
fn read_challenge_1() {
    gamebox::read_file::<gamebox::Challenge>("tests/engines/game/challenge/Deep_Dip_2r1.Map.Gbx")
        .unwrap();
}

#[test]
fn read_challenge_2() {
    gamebox::read_file::<gamebox::Challenge>("tests/engines/game/challenge/Mindor.Map.Gbx")
        .unwrap();
}
