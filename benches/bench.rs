use criterion::{criterion_group, criterion_main, Criterion};
use gamebox::game::ctn::Challenge;

pub fn bench(c: &mut Criterion) {
    c.bench_function("read_map_deep_dip_2r1", |b| {
        b.iter(|| {
            let _map: Challenge =
                gamebox::read_file("tests/files/map/Deep_Dip_2r1.Map.Gbx").unwrap();
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
