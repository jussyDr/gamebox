use criterion::{criterion_group, criterion_main, Criterion};
use gamebox::game::{ctn::Challenge, ItemModel};

pub fn bench(c: &mut Criterion) {
    c.bench_function("read_map_deep_dip_2r1", |b| {
        b.iter(|| {
            let _map: Challenge =
                gamebox::read_file("tests/files/map/Deep_Dip_2r1.Map.Gbx").unwrap();
        })
    });

    c.bench_function("read_map_header_deep_dip_2r1", |b| {
        b.iter(|| {
            let _map: Challenge = gamebox::read::Settings::new()
                .skip_body(true)
                .read_file("tests/files/map/Deep_Dip_2r1.Map.Gbx")
                .unwrap();
        })
    });

    c.bench_function("read_map_mindor", |b| {
        b.iter(|| {
            let _map: Challenge = gamebox::read_file("tests/files/map/Mindor.Map.Gbx").unwrap();
        })
    });

    c.bench_function("read_item_wrh_p_ql_r_3_2", |b| {
        b.iter(|| {
            let _item: ItemModel =
                gamebox::read_file("tests/files/item/WRH_P_QL_R_3_2.Item.Gbx").unwrap();
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
