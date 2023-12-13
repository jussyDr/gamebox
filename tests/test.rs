use std::fs::File;

use gamebox::{
    classes::{item::Item, material::Material, texture::Texture},
    read::HeaderOptions,
};

#[test]
fn read_item() {
    let file = File::open("tests/big_palm_tree_low.Item.Gbx").unwrap();
    let _item: Item = gamebox::read(file).unwrap();
}

#[test]
fn read_material() {
    let file = File::open("tests/TrackWallClips.Material.Gbx").unwrap();

    let _material: Material = gamebox::read::Reader::new()
        .read_header(HeaderOptions::Skip {
            assume_size_zero: true,
        })
        .read(file)
        .unwrap();
}

#[test]
fn read_texture() {
    let file = File::open("tests/TrackWallClips_D.Texture.Gbx").unwrap();

    let _texture: Texture = gamebox::read::Reader::new()
        .read_header(HeaderOptions::Skip {
            assume_size_zero: true,
        })
        .read(file)
        .unwrap();
}
