use criterion::{criterion_group, criterion_main};

mod find_oldest;

criterion_group!(
    benches,
    find_oldest::bench_find_oldest,
    find_oldest::bench_find_oldest_alternative,
    find_oldest::bench_find_oldest_alternative_with_hint
);

criterion_main!(benches);
