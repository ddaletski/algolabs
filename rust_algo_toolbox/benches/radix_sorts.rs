use std::time::Duration;

use algo_toolbox::radix_sort::{lsd, msd};
use criterion::{criterion_group, Criterion, Throughput};
use itertools::Itertools;
use rand::seq::{IteratorRandom, SliceRandom};

fn generate_random_array(n_arrays: usize, max_len: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();

    (0..n_arrays)
        .map(|_| {
            let arr_len = (0..max_len).choose(&mut rng).unwrap();
            (0..arr_len)
                .map(|_| (0u8..=255u8).choose(&mut rng).unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn radix_bench_random(c: &mut Criterion) {
    for n_arrays in [1000, 100000] {
        for max_len in [4, 50] {
            let mut group = c.benchmark_group(format!("radix sort random ({n_arrays}x{max_len})"));
            group
                .throughput(Throughput::Elements(n_arrays as u64))
                .sampling_mode(criterion::SamplingMode::Flat)
                .sample_size(10)
                .measurement_time(Duration::from_secs(5))
                .warm_up_time(Duration::from_secs(1));

            group.bench_with_input("msd", &(n_arrays, max_len), |b, &(n_arrays, max_len)| {
                let mut rand_data = generate_random_array(n_arrays, max_len);
                b.iter(|| msd::sort_vecs(&mut rand_data));
            });

            group.bench_with_input("lsd", &(n_arrays, max_len), |b, &(n_arrays, max_len)| {
                let mut rand_data = generate_random_array(n_arrays, max_len);
                b.iter(|| lsd::sort_vecs(&mut rand_data));
            });

            group.bench_with_input(
                "std::sort",
                &(n_arrays, max_len),
                |b, &(n_arrays, max_len)| {
                    let mut rand_data = generate_random_array(n_arrays, max_len);
                    b.iter(|| rand_data.sort());
                },
            );

            group.finish();
        }
    }
}

fn radix_bench_wordlist(c: &mut Criterion) {
    let words_list = include_str!("words_list.txt")
        .split("\n")
        .map(|s| s.to_owned())
        .collect_vec();

    let mut group = c.benchmark_group(format!("radix sort wordlist"));
    group
        .throughput(Throughput::Elements(words_list.len() as u64))
        .sampling_mode(criterion::SamplingMode::Flat)
        .sample_size(10)
        .measurement_time(Duration::from_secs(5))
        .warm_up_time(Duration::from_secs(1));

    group.bench_function("msd", |b| {
        let mut words = words_list.clone();
        let mut rng = rand::thread_rng();
        words.shuffle(&mut rng);
        b.iter(|| msd::sort_strings(&mut words));
    });

    group.bench_function("lsd", |b| {
        let mut words = words_list.clone();
        let mut rng = rand::thread_rng();
        words.shuffle(&mut rng);
        b.iter(|| lsd::sort_strings(&mut words));
    });

    group.bench_function("std::sort", |b| {
        let mut words = words_list.clone();
        let mut rng = rand::thread_rng();
        words.shuffle(&mut rng);
        b.iter(|| words.sort());
    });

    group.finish();
}

criterion_group!(bench, radix_bench_random, radix_bench_wordlist);
