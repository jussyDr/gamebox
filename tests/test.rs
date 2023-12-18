use std::fs::File;

use gamebox::{
    classes::{color_table::ColorTable, item::Item, material::Material, texture::Texture},
    read::HeaderOptions,
};

#[test]
fn read_item() {
    let file = File::open("tests/files/big_palm_tree_low.Item.Gbx").unwrap();
    let _item: Item = gamebox::read(file).unwrap();
}

#[test]
fn read_material() {
    let file = File::open("tests/files/TrackWallClips.Material.Gbx").unwrap();

    let _material: Material = gamebox::read::Reader::new()
        .read_header(HeaderOptions::Skip {
            assume_size_zero: true,
        })
        .read(file)
        .unwrap();
}

#[test]
fn read_material_2() {
    let file = File::open("tests/files/PlatformTech.Material.Gbx").unwrap();

    let _material: Material = gamebox::read::Reader::new()
        .read_header(HeaderOptions::Skip {
            assume_size_zero: true,
        })
        .read(file)
        .unwrap();
}

#[test]
fn read_texture() {
    let file = File::open("tests/files/TrackWallClips_D.Texture.gbx").unwrap();

    let _texture: Texture = gamebox::read::Reader::new()
        .read_header(HeaderOptions::Skip {
            assume_size_zero: true,
        })
        .read(file)
        .unwrap();
}

#[test]
fn read_color_table() {
    let file = File::open("tests/files/Sport.ColorTable.gbx.json").unwrap();

    let _color_table: ColorTable = gamebox::read::Reader::new()
        .read_header(HeaderOptions::Skip {
            assume_size_zero: true,
        })
        .read(file)
        .unwrap();
}
