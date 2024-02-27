use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::Rng;

use algo_toolbox::union_find::{DenseUF, UnionFind};

fn generate_random_edges(n_nodes: usize, n_edges: usize) -> Vec<(usize, usize)> {
    let mut rng = rand::thread_rng();

    let mut pairs = (0..n_edges)
        .map(|_| {
            let id1 = rng.gen_range(0..n_nodes);
            let id2 = rng.gen_range(0..n_nodes);
            (id1, id2)
        })
        .collect_vec();

    pairs.shuffle(&mut rng);

    pairs
}

fn bench_union(mut uf: DenseUF, pairs: &Vec<(usize, usize)>) {
    for (id1, id2) in pairs {
        uf.join(*id1, *id2);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("union");
    for n_nodes in (4..=6).map(|p| usize::pow(10, p)) {
        let n_edges = n_nodes * 4;

        group
            .throughput(Throughput::Elements(n_edges as u64))
            .sampling_mode(criterion::SamplingMode::Flat)
            .sample_size(10)
            .warm_up_time(Duration::from_secs(1));

        group.bench_with_input(
            format!("union {}-{}", n_nodes, n_edges),
            &(n_nodes, n_edges),
            |b, &(n_nodes, n_edges)| {
                let pairs = &generate_random_edges(n_nodes, n_edges);
                let uf = DenseUF::new(n_nodes);
                b.iter(|| bench_union(uf.clone(), pairs));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
