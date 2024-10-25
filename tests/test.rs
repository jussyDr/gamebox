#[test]
fn read_map_1() {
    gamebox::read_file::<gamebox::Challenge>("tests/files/map/Deep_Dip_2r1.Map.Gbx").unwrap();
}

#[test]
fn read_map_2() {
    gamebox::read_file::<gamebox::Challenge>("tests/files/map/Mindor.Map.Gbx").unwrap();
}

#[test]
fn read_map_3() {
    gamebox::read_file::<gamebox::Challenge>("tests/files/map/MIDNIGHT METROPOLIS.Map.Gbx")
        .unwrap();
}

#[test]
fn read_item_1() {
    gamebox::read_file::<gamebox::ItemModel>("tests/files/item/CP-stripe.Item.Gbx").unwrap();
}
