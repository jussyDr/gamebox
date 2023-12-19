#![feature(test)]

extern crate test;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use gamebox::{
    classes::{
        color_table::ColorTable, material::Material, prefab::Prefab, texture::Texture,
        veget_tree_model::VegetTreeModel, Item,
    },
    read::{HeaderOptions, Readable},
};
use test::{test_main, ShouldPanic, TestDesc, TestDescAndFn, TestFn, TestName, TestType};
use walkdir::WalkDir;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let mut tests = vec![];

    read_extracted_file_tests::<ColorTable>(&mut tests, "tests/files/color_table");
    read_extracted_file_tests::<Item>(&mut tests, "tests/files/item/game");
    read_file_tests::<Item>(&mut tests, "tests/files/item/custom");
    read_extracted_file_tests::<Material>(&mut tests, "tests/files/material");
    read_extracted_file_tests::<Prefab>(&mut tests, "tests/files/prefab");
    read_extracted_file_tests::<Texture>(&mut tests, "tests/files/texture");
    read_extracted_file_tests::<VegetTreeModel>(&mut tests, "tests/files/veget_tree_model");

    // read_extracted_file_tests_recursive(&mut tests, "C:/Users/Justin/Projects/tm-files");

    test_main(&args, tests, None);
}

fn read_file_tests<T: Readable>(tests: &mut Vec<TestDescAndFn>, dir_path: impl AsRef<Path>) {
    read_file_tests_inner(tests, dir_path, read_file::<T>)
}

fn read_extracted_file_tests<T: Readable>(
    tests: &mut Vec<TestDescAndFn>,
    dir_path: impl AsRef<Path>,
) {
    read_file_tests_inner(tests, dir_path, read_extracted_file::<T>)
}

fn read_file_tests_inner(
    tests: &mut Vec<TestDescAndFn>,
    dir_path: impl AsRef<Path>,
    read_fn: fn(PathBuf),
) {
    for entry in fs::read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();

        let file_name = entry.file_name().to_str().unwrap().to_owned();

        let test = TestDescAndFn {
            desc: TestDesc {
                name: TestName::DynTestName(file_name),
                ignore: false,
                ignore_message: None,
                source_file: "",
                start_line: 0,
                start_col: 0,
                end_line: 0,
                end_col: 0,
                should_panic: ShouldPanic::No,
                compile_fail: false,
                no_run: false,
                test_type: TestType::IntegrationTest,
            },
            testfn: TestFn::DynTestFn(Box::new(move || {
                read_fn(entry.path());

                Ok(())
            })),
        };

        tests.push(test);
    }
}

fn read_file<T: Readable>(path: impl AsRef<Path>) {
    gamebox::read_file::<T>(path).unwrap();
}

fn read_extracted_file<T: Readable>(path: impl AsRef<Path>) {
    gamebox::read::Reader::new()
        .read_header(HeaderOptions::Skip {
            assume_size_zero: true,
        })
        .read_file::<T>(path)
        .unwrap();
}

fn read_extracted_file_tests_recursive(tests: &mut Vec<TestDescAndFn>, dir_path: impl AsRef<Path>) {
    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();

        if !entry.file_type().is_file() {
            continue;
        }

        let file_name = entry.file_name().to_str().unwrap().to_owned();
        let file_extension = file_name.split_once('.').unwrap().1.to_owned();

        let read_fn = match file_extension.to_lowercase().as_str() {
            "colortable.gbx.json" => |path: &Path| read_extracted_file::<ColorTable>(path),
            "item.gbx" => |path: &Path| read_extracted_file::<Item>(path),
            "material.gbx" => |path: &Path| read_extracted_file::<Material>(path),
            "prefab.gbx" => |path: &Path| read_extracted_file::<Prefab>(path),
            "texture.gbx" => |path: &Path| read_extracted_file::<Texture>(path),
            "vegettreemodel.gbx" => |path: &Path| read_extracted_file::<VegetTreeModel>(path),
            _ => continue,
        };

        let test = TestDescAndFn {
            desc: TestDesc {
                name: TestName::DynTestName(file_name),
                ignore: false,
                ignore_message: None,
                source_file: "",
                start_line: 0,
                start_col: 0,
                end_line: 0,
                end_col: 0,
                should_panic: ShouldPanic::No,
                compile_fail: false,
                no_run: false,
                test_type: TestType::IntegrationTest,
            },
            testfn: TestFn::DynTestFn(Box::new(move || {
                read_fn(entry.path());

                Ok(())
            })),
        };

        tests.push(test);
    }
}
