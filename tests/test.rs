#![feature(test)]

extern crate test;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use gamebox::{
    classes::{
        color_table::ColorTable, material::Material, prefab::Prefab, texture::Texture,
        veget_tree_model::VegetTreeModel,
    },
    read::{HeaderOptions, Readable},
    write::Writable,
    Item, Map,
};
use glob::glob;
use test::{test_main, ShouldPanic, TestDesc, TestDescAndFn, TestFn, TestName, TestType};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let mut tests = vec![];

    // for entry in glob("C:/Users/Justin/Downloads/TM-Items_V9/**/*.Item.Gbx")
    //     .expect("Failed to read glob pattern")
    // {
    //     let entry = entry.unwrap();

    //     let file_name = entry.file_name().unwrap().to_str().unwrap().to_owned();

    //     let test = create_test(
    //         format!("read {file_name}"),
    //         Box::new(move || {
    //             read_file::<Item>(&entry);

    //             Ok(())
    //         }),
    //     );

    //     tests.push(test);
    // }

    add_read_extracted_file_tests::<ColorTable>(&mut tests, "tests/files/color_table");
    add_read_extracted_file_tests::<Item>(&mut tests, "tests/files/item/game");
    add_read_file_tests::<Item>(&mut tests, "tests/files/item/custom");
    add_read_file_tests::<Map>(&mut tests, "tests/files/map");
    add_read_extracted_file_tests::<Material>(&mut tests, "tests/files/material");
    add_read_extracted_file_tests::<Prefab>(&mut tests, "tests/files/prefab");
    add_read_extracted_file_tests::<Texture>(&mut tests, "tests/files/texture");
    add_read_extracted_file_tests::<VegetTreeModel>(&mut tests, "tests/files/veget_tree_model");
    add_write_read_default_test::<Item>(&mut tests, "Item");
    add_write_read_default_test::<Map>(&mut tests, "Map");

    test_main(&args, tests, None);
}

fn add_read_file_tests<T: Readable>(tests: &mut Vec<TestDescAndFn>, dir_path: impl AsRef<Path>) {
    add_read_file_tests_inner(tests, dir_path, read_file::<T>)
}

fn add_read_extracted_file_tests<T: Readable>(
    tests: &mut Vec<TestDescAndFn>,
    dir_path: impl AsRef<Path>,
) {
    add_read_file_tests_inner(tests, dir_path, read_extracted_file::<T>)
}

fn add_read_file_tests_inner(
    tests: &mut Vec<TestDescAndFn>,
    dir_path: impl AsRef<Path>,
    read_fn: fn(PathBuf),
) {
    for entry in fs::read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();

        let file_name = entry.file_name().to_str().unwrap().to_owned();

        let test = create_test(
            format!("read {file_name}"),
            Box::new(move || {
                read_fn(entry.path());

                Ok(())
            }),
        );

        tests.push(test);
    }
}

fn add_write_read_default_test<T: Default + Readable + Writable>(
    tests: &mut Vec<TestDescAndFn>,
    name: &str,
) {
    let test = create_test(
        format!("write read default {name}"),
        Box::new(move || {
            write_read_default::<T>();

            Ok(())
        }),
    );

    tests.push(test);
}

fn create_test(
    name: String,
    test_fn: Box<dyn FnOnce() -> Result<(), String> + Send>,
) -> TestDescAndFn {
    TestDescAndFn {
        desc: TestDesc {
            name: TestName::DynTestName(name),
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
        testfn: TestFn::DynTestFn(test_fn),
    }
}

fn read_file<T: Readable>(path: impl AsRef<Path>) {
    gamebox::read_file::<T>(path).unwrap();
}

fn read_extracted_file<T: Readable>(path: impl AsRef<Path>) {
    gamebox::Reader::new()
        .read_header(HeaderOptions::Skip {
            assume_size_zero: true,
        })
        .read_file::<T>(path)
        .unwrap();
}

fn write_read_default<T: Default + Readable + Writable>() {
    let mut buf = vec![];
    gamebox::write(&T::default(), &mut buf).unwrap();
    gamebox::read::<T>(buf.as_slice()).unwrap();
}
