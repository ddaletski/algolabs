use criterion::{black_box, criterion_group, criterion_main, Criterion};
use data_compression::{bwt::BWT, huffman::HuffmanTransform, mtf::MTF, DataTransformer};

const TEST_DATA: &[u8] = include_bytes!("lorem.txt");

fn huffman_benchmark(c: &mut Criterion) {
    c.bench_function("huffman", |b| {
        b.iter(|| HuffmanTransform::default().transform(black_box(TEST_DATA)))
    });
}

fn mtf_benchmark(c: &mut Criterion) {
    c.bench_function("mtf", |b| {
        b.iter(|| MTF::default().transform(black_box(TEST_DATA)))
    });
}

fn bwt_benchmark(c: &mut Criterion) {
    c.bench_function("bwt", |b| {
        b.iter(|| BWT::default().transform(black_box(TEST_DATA)))
    });
}

criterion_group!(benches, huffman_benchmark, mtf_benchmark, bwt_benchmark);
criterion_main!(benches);
