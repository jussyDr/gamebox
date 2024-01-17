#![feature(test)]

extern crate test;

use std::{fs::File, io::BufReader};

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
    b.iter(|| {
        let file = File::open("tests/files/map/Mindor.Map.Gbx").unwrap();
        let reader = BufReader::new(file);
        GbxFile::read(reader, false).unwrap();
    })
}
