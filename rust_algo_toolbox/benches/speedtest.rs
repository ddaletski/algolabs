use criterion::criterion_main;

mod union_find;
mod radix_sorts;
mod priority_queues;

criterion_main!(union_find::bench, radix_sorts::bench, priority_queues::bench);
