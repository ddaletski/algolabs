use std::time::Duration;

use algo_toolbox::radix_sort::{lsd, msd, radix_qsort};
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

const PARTS: [&'static str; 6] = ["abcd", "lol", "kek", "chebureck", "hahaha", "00000000"];
fn generate_non_random_array(n_arrays: usize, n_words: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();

    (0..n_arrays)
        .map(|_| {
            PARTS
                .choose_multiple(&mut rng, n_words)
                .map(|&s| s.bytes())
                .flatten()
                .collect_vec()
        })
        .collect_vec()
}

fn radix_bench_random(c: &mut Criterion) {
    let sets: Vec<(&'static str, Box<dyn Fn() -> Vec<Vec<u8>>>)> = vec![
        (
            "many short random arrays",
            Box::new(|| generate_random_array(1000000, 4)),
        ),
        (
            "few long random arrays",
            Box::new(|| generate_random_array(100, 100000)),
        ),
        (
            "many non-random arrays",
            Box::new(|| generate_non_random_array(100000, 100)),
        ),
    ];

    for (label, generator) in sets {
        let mut group = c.benchmark_group(format!("radix sort of {label}"));
        group
            .sampling_mode(criterion::SamplingMode::Flat)
            .sample_size(10)
            .measurement_time(Duration::from_secs(5))
            .warm_up_time(Duration::from_secs(1));

        group.bench_function("msd", |b| {
            let mut rand_data = generator();
            b.iter(|| msd::sort_vecs(&mut rand_data));
        });

        group.bench_function("lsd", |b| {
            let mut rand_data = generator();
            b.iter(|| lsd::sort_vecs(&mut rand_data));
        });

        group.bench_function("3w-rqsort", |b| {
            let mut rand_data = generator();
            b.iter(|| radix_qsort::sort_vecs(&mut rand_data));
        });

        group.bench_function("std::sort", |b| {
            let mut rand_data = generator();
            b.iter(|| rand_data.sort_unstable());
        });

        group.finish();
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

    group.bench_function("3w-qsort", |b| {
        let mut words = words_list.clone();
        let mut rng = rand::thread_rng();
        words.shuffle(&mut rng);
        b.iter(|| radix_qsort::sort_strings(&mut words));
    });

    group.bench_function("std::sort", |b| {
        let mut words = words_list.clone();
        let mut rng = rand::thread_rng();
        words.shuffle(&mut rng);
        b.iter(|| words.sort_unstable());
    });

    group.finish();
}

criterion_group!(bench, radix_bench_random, radix_bench_wordlist);
