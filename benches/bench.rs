use criterion::{criterion_group, criterion_main};

mod cache_line;
mod find_oldest;

criterion_group!(
    find_oldest_benches,
    find_oldest::bench_find_oldest,
    find_oldest::bench_find_oldest_alternative,
    find_oldest::bench_find_oldest_alternative_with_hint
);

criterion_group!(
    cache_line_benches,
    cache_line::bench_not_padded,
    cache_line::bench_padded,
    cache_line::bench_crossbeam_padded
);

criterion_main!(find_oldest_benches, cache_line_benches);
