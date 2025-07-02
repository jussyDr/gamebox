#![feature(test)]

extern crate test;

use gamebox::class::plug::solid_2_model::Solid2Model;
use test::Bencher;

#[bench]
fn read_mesh(b: &mut Bencher) {
    b.iter(|| {
        let _mesh: Solid2Model =
            gamebox::read_file("tests/files/mesh/Stade1536v2.Mesh.Gbx").unwrap();
    });
}
