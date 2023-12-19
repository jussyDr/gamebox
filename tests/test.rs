use std::{
    fs::{self, File},
    io::{self, BufReader},
};

use gamebox::{
    classes::{
        color_table::ColorTable, material::Material, prefab::Prefab, texture::Texture,
        veget_tree_model::VegetTreeModel, Item,
    },
    read::{HeaderOptions, Readable},
};
use libtest_mimic::{Arguments, Trial};

fn main() -> io::Result<()> {
    let args = Arguments::from_args();

    let mut tests = vec![];

    test_read_extracted_folder::<ColorTable>(&mut tests, "tests/files/color_table")?;
    test_read_folder::<Item>(&mut tests, "tests/files/item/custom")?;
    test_read_extracted_folder::<Item>(&mut tests, "tests/files/item/game")?;
    test_read_extracted_folder::<Material>(&mut tests, "tests/files/material")?;
    test_read_extracted_folder::<Prefab>(&mut tests, "tests/files/prefab")?;
    test_read_extracted_folder::<Texture>(&mut tests, "tests/files/texture")?;
    test_read_extracted_folder::<VegetTreeModel>(&mut tests, "tests/files/veget_tree_model")?;

    libtest_mimic::run(&args, tests).exit();
}

fn test_read_folder<T: Readable>(tests: &mut Vec<Trial>, folder_path: &str) -> io::Result<()> {
    for entry in fs::read_dir(folder_path)? {
        let file_path = entry?.path();

        let file_name = file_path.file_name().unwrap().to_str().unwrap().to_owned();

        let test = Trial::test(format!("read {file_name}"), || {
            let file = File::open(file_path).unwrap();
            let reader = BufReader::new(file);
            gamebox::read::<T>(reader).unwrap();

            Ok(())
        });

        tests.push(test)
    }

    Ok(())
}

fn test_read_extracted_folder<T: Readable>(
    tests: &mut Vec<Trial>,
    folder_path: &str,
) -> io::Result<()> {
    for entry in fs::read_dir(folder_path)? {
        let file_path = entry?.path();

        let file_name = file_path.file_name().unwrap().to_str().unwrap().to_owned();

        let test = Trial::test(format!("read {file_name}"), || {
            let file = File::open(file_path).unwrap();
            let reader = BufReader::new(file);

            gamebox::read::Reader::new()
                .read_header(HeaderOptions::Skip {
                    assume_size_zero: true,
                })
                .read::<T>(reader)
                .unwrap();

            Ok(())
        });

        tests.push(test)
    }

    Ok(())
}

// #[test]
// fn read_game_data() {
//     for entry in WalkDir::new("C:/Users/Justin/Projects/tm-files") {
//         let entry = entry.unwrap();

//         if entry.file_type().is_dir() {
//             continue;
//         }

//         let path = entry.path().to_str().unwrap();
//         let (_, extension) = path.split_once('.').unwrap();

//         println!("{path}");

//         match extension.to_lowercase().as_str() {
//             "colortable.gbx.json" => {
//                 test_read_extracted_file::<ColorTable>(path);
//             }
//             "item.gbx" => {
//                 test_read_extracted_file::<Item>(path);
//             }
//             "material.gbx" | "material.gbx_476" | "material.gbx_498" | "material.gbx_511"
//             | "material.gbx_520" | "material.gbx_531" | "material.gbx_537" | "material.gbx_542" => {
//                 test_read_extracted_file::<Material>(path);
//             }
//             "prefab.gbx" => {
//                 test_read_extracted_file::<Prefab>(path);
//             }
//             "texture.gbx" => {
//                 test_read_extracted_file::<Texture>(path);
//             }
//             "vegettreemodel.gbx" => {
//                 test_read_extracted_file::<VegetTreeModel>(path);
//             }
//             "light.gbx" => {}
//             "shape.gbx" => {}
//             "imagegen.gbx" => {}

//             "fxsys.gbx" => {}
//             "terrainmodifier.gbx" | "terrainmodifier .gbx" => {}
//             "dds" => {}
//             "tga" => {}
//             "gbx" => {}
//             "gameskin.gbx" => {}
//             "kinematicconstraint.gbx" => {}
//             _ => panic!("{extension}"),
//         }
//     }
// }
