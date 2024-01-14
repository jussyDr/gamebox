#![feature(test)]

extern crate test;

use gamebox::Map;
use test::Bencher;

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| {
        gamebox::read_file::<Map>("tests/files/map/Mindor.Map.Gbx").unwrap();
    })
}
