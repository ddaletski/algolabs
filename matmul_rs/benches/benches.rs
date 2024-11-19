use criterion::{criterion_group, criterion_main, Criterion};
use matmul_rs::{
    matmul, mm_loop_reorder, mm_loop_reorder_rayon, mm_loop_reorder_safe, mm_tiling, mm_tiling_rayon, mm_tiling_rayon_vectorized, Mat, MatMulImpl
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{
    hint::black_box,
    sync::{LazyLock, RwLock},
    time::Duration,
};

static RNG: LazyLock<RwLock<StdRng>> = LazyLock::new(|| StdRng::seed_from_u64(0).into());

fn rand_mat(h: usize, w: usize) -> Mat {
    let mut rng = RNG.write().unwrap();

    let data = (0..h * w).map(|_| rng.gen_range(0.0..1.0)).collect();
    Mat {
        data,
        height: h,
        width: w,
    }
}

fn zero_mat(h: usize, w: usize) -> Mat {
    Mat {
        data: vec![0.0; h * w],
        height: h,
        width: w,
    }
}

fn criterion_benchmark(crit: &mut Criterion) {
    let mut group = crit.benchmark_group("matmul");
    group
        .sample_size(10)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3));

    let functions = [
        ("loop reorder safe", MatMulImpl::Safe(mm_loop_reorder_safe)),
        ("loop reorder", MatMulImpl::Unsafe(mm_loop_reorder)),
        (
            "loop reorder | rayon",
            MatMulImpl::Unsafe(mm_loop_reorder_rayon),
        ),
        ("tiling", MatMulImpl::Unsafe(mm_tiling)),
        ("tiling | rayon", MatMulImpl::Unsafe(mm_tiling_rayon)),
        (
            "tiling | rayon | vectorized",
            MatMulImpl::Unsafe(mm_tiling_rayon_vectorized),
        ),
    ];

    for (name, f) in functions {
        let size = 1024;

        let mut c = zero_mat(size, size);

        group.bench_function(name.to_string(), |bencher| {
            let a = rand_mat(size, size);
            let b = rand_mat(size, size);
            bencher.iter(|| black_box(matmul(&f, &a, &b, &mut c)))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
