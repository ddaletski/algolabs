use criterion::criterion_main;

mod union_find;
mod radix_sorts;

criterion_main!(union_find::bench, radix_sorts::bench);
