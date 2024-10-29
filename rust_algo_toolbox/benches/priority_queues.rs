use algo_toolbox::pqueue::dway_heap::DWayMaxHeap;
use std::{collections::BinaryHeap, time::Duration};

use criterion::{criterion_group, BenchmarkId, Criterion};
use rand::{Rng, SeedableRng};

fn random_array(n: usize) -> Vec<i32> {
    let rng = rand::rngs::StdRng::from_seed([0; 32]);
    let distr = rand::distributions::Uniform::new_inclusive(i32::MIN, i32::MAX);

    rng.sample_iter(distr).take(n).collect()
}

fn increasing_array(n: usize) -> Vec<i32> {
    (0..n).map(|v| v as i32).collect()
}

fn decreasing_array(n: usize) -> Vec<i32> {
    (0..n).rev().map(|v| v as i32).collect()
}

fn pqueue_bench(c: &mut Criterion) {
    let datasets: [(&str, Box<dyn Fn(usize) -> Vec<i32>>); 3] = [
        ("increasing", Box::new(increasing_array)),
        ("decreasing", Box::new(decreasing_array)),
        ("random", Box::new(random_array)),
    ];

    for (dataset_name, data_gen) in datasets {
        bench_on_dataset(c, dataset_name, data_gen);
    }
}

fn bench_on_dataset(
    c: &mut Criterion,
    dataset_name: &str,
    data_gen: Box<dyn Fn(usize) -> Vec<i32>>,
) {
    let mut group = c.benchmark_group(format!("heap sort {dataset_name}"));

    group
        .sampling_mode(criterion::SamplingMode::Flat)
        .sample_size(20)
        .measurement_time(Duration::from_secs(2))
        .warm_up_time(Duration::from_millis(500));

    for i in [100_000, 300_000, 600_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::new("std::BinaryHeap", i), &i, |b, i| {
            let data = data_gen(*i);
            b.iter(|| {
                let heap: BinaryHeap<i32> = data.iter().cloned().collect();
                let _: Vec<i32> = heap.clone().into_sorted_vec();
            });
        });

        group.bench_with_input(BenchmarkId::new("4-way heap", i), &i, |b, i| {
            let data = data_gen(*i);
            b.iter(|| {
                let heap: DWayMaxHeap<i32, 4> = data.iter().cloned().collect();
                let _: Vec<i32> = heap.clone().into_sorted_vec();
            });
        });

        group.bench_with_input(BenchmarkId::new("8-way heap", i), &i, |b, i| {
            let data = data_gen(*i);
            b.iter(|| {
                let heap: DWayMaxHeap<i32, 8> = data.iter().cloned().collect();
                let _: Vec<i32> = heap.clone().into_sorted_vec();
            });
        });
    }

    group.finish();
}

criterion_group!(bench, pqueue_bench);
