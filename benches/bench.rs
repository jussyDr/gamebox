#![feature(test)]

extern crate test;

use std::{
    fs::{self},
    io::Cursor,
};

use gamebox::{read::GbxFile, Map};
use test::Bencher;

#[bench]
fn read_map(b: &mut Bencher) {
    b.iter(|| {
        gamebox::read_file::<Map>("tests/files/map/Mindor.Map.Gbx").unwrap();
    })
}

#[bench]
fn read_map_file(b: &mut Bencher) {
    let data = fs::read("tests/files/map/Mindor.Map.Gbx").unwrap();

    b.iter(|| {
        GbxFile::read(Cursor::new(data.as_slice()), false).unwrap();
    })
}
