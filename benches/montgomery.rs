use criterion::{black_box, criterion_group, criterion_main, Criterion};
use montgomery_reduction::arith::school_method;
use montgomery_reduction::emmart;
use montgomery_reduction::{acar, NP0, P, U52_NP0, U52_P};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn bench_acar(c: &mut Criterion) {
    let mut group = c.benchmark_group("Acar");

    // Generate and print a random seed
    let seed: u64 = rand::random();
    println!("Using random seed: {}", seed);
    let mut rng = StdRng::seed_from_u64(seed);

    // Generate random test case
    let a = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let b = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];

    let c = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let d = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];

    println!("Random test values:");
    println!("a = {:?}", a);
    println!("b = {:?}", b);

    group.bench_function("sos_random", |bencher| {
        bencher.iter(|| acar::sos(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("fios_random", |bencher| {
        bencher.iter(|| acar::fios(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("cios_random", |bencher| {
        bencher.iter(|| acar::cios(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("cios_opt_random", |bencher| {
        bencher.iter(|| acar::cios_opt(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("cios_opt_seq_random", |bencher| {
        bencher.iter(|| acar::cios_opt_seq(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("cios_opt_sat_random", |bencher| {
        bencher.iter(|| {
            acar::cios_opt_sat(
                black_box(a),
                black_box(b),
                black_box(c),
                black_box(d),
                P,
                NP0,
            )
        })
    });

    group.bench_function("mul_school_method", |bencher| {
        bencher.iter(|| school_method(black_box(a), black_box(b)))
    });
}

fn bench_emmart(c: &mut Criterion) {
    let mut group = c.benchmark_group("Emmart");

    // Generate and print a random seed
    let seed: u64 = rand::random();
    println!("Using random seed: {}", seed);
    let mut rng = StdRng::seed_from_u64(seed);

    // SET ROUND TO ZERO BENCHES
    emmart::set_round_to_zero();
    let a = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let b = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let c = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let d = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let e = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let f = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let g = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let h = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let i = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let j = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];

    let a_float = a.map(|x| x as f64);
    let b_float = b.map(|x| x as f64);

    group.bench_function("mul_sampled_product_masked_random", |bencher| {
        bencher
            .iter(|| emmart::paper::sampled_product_masked(black_box(a_float), black_box(b_float)))
    });
    group.bench_function("mul_sampled_product_random", |bencher| {
        bencher.iter(|| emmart::paper::sampled_product(black_box(a_float), black_box(b_float)))
    });

    group.bench_function("cios_opt_random", |bencher| {
        bencher.iter(|| emmart::cios_opt(black_box(a), black_box(b), U52_P, U52_NP0))
    });

    group.bench_function("fios_opt_random", |bencher| {
        bencher.iter(|| emmart::fios_opt(black_box(a), black_box(b), U52_P, U52_NP0))
    });
    group.bench_function("fios_opt_sub_random", |bencher| {
        bencher.iter(|| emmart::fios_opt_sub(black_box(a), black_box(b), U52_P, U52_NP0))
    });
    group.bench_function("fios_opt_sub_sat_random", |bencher| {
        bencher.iter(|| {
            emmart::fios_opt_sub_sat(
                black_box(a),
                black_box(b),
                black_box(c),
                black_box(d),
                U52_P,
                U52_NP0,
            )
        })
    });
    group.bench_function("fios_opt_sub_simd_random", |bencher| {
        bencher.iter(|| {
            emmart::fios_opt_sub_simd(
                black_box(a),
                black_box(b),
                black_box(c),
                black_box(d),
                U52_P,
                U52_NP0,
            )
        })
    });
    group.bench_function("fios_opt_sub_simd_sat_random", |bencher| {
        bencher.iter(|| {
            emmart::fios_opt_sub_simd_sat(
                black_box(a),
                black_box(b),
                black_box(c),
                black_box(d),
                black_box(e),
                black_box(f),
                black_box(g),
                black_box(h),
                U52_P,
                U52_NP0,
            )
        })
    });
    group.bench_function("fios_opt_sub_simd_sat_seq_random", |bencher| {
        bencher.iter(|| {
            emmart::fios_opt_sub_simd_sat_seq(
                black_box(a),
                black_box(b),
                black_box(c),
                black_box(d),
                black_box(e),
                black_box(f),
                black_box(g),
                black_box(h),
                black_box(i),
                black_box(j),
                U52_P,
                U52_NP0,
            )
        })
    });
    group.bench_function("cios_opt_sub_f64_random", |bencher| {
        bencher.iter(|| emmart::cios_opt_sub(black_box(a), black_box(b), U52_P, U52_NP0))
    });

    // Set up for sampled_product_masked benchmark

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(5000)
        .warm_up_time(std::time::Duration::new(3,0))
        .measurement_time(std::time::Duration::new(10,0));
    targets = bench_acar, bench_emmart
);
criterion_main!(benches);
