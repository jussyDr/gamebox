use std::fs::File;

use gamebox::classes::{item::Item, material::Material};

#[test]
fn read_item() {
    let file = File::open("tests/big_palm_tree_low.Item.Gbx").unwrap();
    let _item: Item = gamebox::read(file).unwrap();
}

#[test]
fn read_material() {
    let file = File::open("tests/TrackWallClips.Material.Gbx").unwrap();

    let _item: Material = gamebox::read::Reader::new()
        .assume_header_size_zero(true)
        .read(file)
        .unwrap();
}
