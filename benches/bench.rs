use criterion::*;

use ore_rs::{scheme::bit2::OREAES128, ORECipher, OREEncrypt};

use tfhe::{generate_keys, prelude::*, set_server_key, ConfigBuilder, FheUint64};

fn benchmark(c: &mut Criterion) {
    let config = ConfigBuilder::all_disabled()
        .enable_default_integers()
        .build();

    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    let a = FheUint64::encrypt(100_u64, &client_key);
    let b = FheUint64::encrypt(200_u64, &client_key);

    let mut group = c.benchmark_group("tfhe");

    group.bench_function("encrypt", |be| {
        be.iter(|| {
            let res = FheUint64::encrypt(300_u64, &client_key);
            black_box(res);
        })
    });

    group.bench_function("a == b", |be| {
        be.iter(|| {
            let res = a.eq(&b);
            black_box(res);
        })
    });

    group.bench_function("a > b", |be| {
        be.iter(|| {
            let res = a.gt(&b);
            black_box(res);
        })
    });

    group.bench_function("a < b", |be| {
        be.iter(|| {
            let res = a.lt(&b);
            black_box(res);
        })
    });

    group.bench_function("a >= a", |be| {
        be.iter(|| {
            let res = a.ge(&a);
            black_box(res);
        })
    });

    group.bench_function("a <= a", |be| {
        be.iter(|| {
            let res = a.le(&a);
            black_box(res);
        })
    });

    group.finish();

    let mut group = c.benchmark_group("ore");

    let k1 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let k2 = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

    let mut ore: OREAES128 = ORECipher::init(&k1, &k2).unwrap();

    let a = 100u64.encrypt(&mut ore).unwrap();
    let b = 200u64.encrypt(&mut ore).unwrap();

    group.bench_function("encrypt", |be| {
        be.iter(|| {
            let res = 300_u64.encrypt(&mut ore).unwrap();
            black_box(res);
        })
    });

    group.bench_function("a == b", |be| {
        be.iter(|| {
            let res = a.eq(&b);
            black_box(res);
        })
    });

    group.bench_function("a > b", |be| {
        be.iter(|| {
            let res = a.gt(&b);
            black_box(res);
        })
    });

    group.bench_function("a < b", |be| {
        be.iter(|| {
            let res = a.lt(&b);
            black_box(res);
        })
    });

    group.bench_function("a >= a", |be| {
        be.iter(|| {
            let res = a.ge(&a);
            black_box(res);
        })
    });

    group.bench_function("a <= a", |be| {
        be.iter(|| {
            let res = a.le(&a);
            black_box(res);
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
