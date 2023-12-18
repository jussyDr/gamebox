use std::{fs::File, io::BufReader, path::Path};

use gamebox::{
    classes::{
        color_table::ColorTable, item::Item, material::Material, prefab::Prefab, texture::Texture,
        veget_tree_model::VegetTreeModel,
    },
    read::{HeaderOptions, Readable},
};

#[test]
fn read_item() {
    test_read_file::<Item>("tests/files/big_palm_tree_low.Item.Gbx");
}

#[test]
fn read_item_2() {
    test_read_extracted_file::<Item>("tests/files/Fall.Item.Gbx");
}

#[test]
fn read_material() {
    test_read_extracted_file::<Material>("tests/files/TrackWallClips.Material.Gbx");
}

#[test]
fn read_material_2() {
    test_read_extracted_file::<Material>("tests/files/PlatformTech.Material.Gbx");
}

#[test]
fn read_texture() {
    test_read_extracted_file::<Texture>("tests/files/TrackWallClips_D.Texture.gbx");
}

#[test]
fn read_color_table() {
    test_read_file::<ColorTable>("tests/files/Sport.ColorTable.gbx.json");
}

#[test]
fn read_veget_tree_model() {
    test_read_extracted_file::<VegetTreeModel>("tests/files/FallTreeMedium.VegetTreeModel.gbx");
}

#[test]
fn read_prefab() {
    test_read_extracted_file::<Prefab>("tests/files/Fall.Prefab.gbx");
}

fn test_read_file<T: Readable>(path: impl AsRef<Path>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    gamebox::read::<T>(reader).unwrap();
}

fn test_read_extracted_file<T: Readable>(path: impl AsRef<Path>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    gamebox::read::Reader::new()
        .read_header(HeaderOptions::Skip {
            assume_size_zero: true,
        })
        .read::<T>(reader)
        .unwrap();
}
