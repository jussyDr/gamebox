#![feature(test)]

extern crate test;

use gamebox::{Challenge, read_file};
use test::Bencher;

#[bench]
fn read_map(b: &mut Bencher) {
    b.iter(|| {
        let _map: Challenge = read_file("tests/files/map/Alive.Map.Gbx").unwrap();
    });
}
