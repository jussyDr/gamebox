use std::{fs::File, io::BufReader, path::Path};

use gamebox::{
    classes::{
        color_table::ColorTable, item::Item, material::Material, prefab::Prefab, texture::Texture,
        veget_tree_model::VegetTreeModel,
    },
    read::{HeaderOptions, Readable},
};
use walkdir::WalkDir;

#[test]
fn read_item() {
    test_read_file::<Item>("tests/files/big_palm_tree_low.Item.Gbx");
}

#[test]
fn read_item_2() {
    test_read_extracted_file::<Item>("tests/files/Fall.Item.Gbx");
}

#[test]
fn read_item_3() {
    test_read_extracted_file::<Item>("tests/files/GateCheckpointCenter16m.Item.Gbx");
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

#[test]
fn read_game_data() {
    for entry in WalkDir::new("C:/Users/Justin/Projects/tm-files") {
        let entry = entry.unwrap();

        if entry.file_type().is_dir() {
            continue;
        }

        let path = entry.path().to_str().unwrap();
        let (_, extension) = path.split_once('.').unwrap();

        println!("{path}");

        match extension.to_lowercase().as_str() {
            "colortable.gbx.json" => {
                test_read_extracted_file::<ColorTable>(path);
            }
            "item.gbx" => {
                test_read_extracted_file::<Item>(path);
            }
            "material.gbx" | "material.gbx_476" | "material.gbx_498" | "material.gbx_511"
            | "material.gbx_520" | "material.gbx_531" | "material.gbx_537" | "material.gbx_542" => {
                test_read_extracted_file::<Material>(path);
            }
            "prefab.gbx" => {
                test_read_extracted_file::<Prefab>(path);
            }
            "texture.gbx" => {
                test_read_extracted_file::<Texture>(path);
            }
            "vegettreemodel.gbx" => {
                test_read_extracted_file::<VegetTreeModel>(path);
            }
            "light.gbx" => {}
            "shape.gbx" => {}
            "imagegen.gbx" => {}

            "fxsys.gbx" => {}
            "terrainmodifier.gbx" | "terrainmodifier .gbx" => {}
            "dds" => {}
            "tga" => {}
            "gbx" => {}
            "gameskin.gbx" => {}
            "kinematicconstraint.gbx" => {}
            _ => panic!("{extension}"),
        }
    }
}
