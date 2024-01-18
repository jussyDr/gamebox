#![feature(test)]

extern crate test;

use gamebox::Map;
use test::Bencher;

#[bench]
fn read_map(b: &mut Bencher) {
    b.iter(|| {
        gamebox::read_file::<Map>("tests/files/map/Mindor.Map.Gbx").unwrap();
    })
}
