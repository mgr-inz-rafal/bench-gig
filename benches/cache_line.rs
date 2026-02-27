use bench_core::cache_line::{PaddedCacheLine, SharedCacheLine, run_padded, run_shared};
use criterion::{Criterion, criterion_group, criterion_main};
use std::{hint::black_box, sync::Arc};

pub fn bench_not_padded(c: &mut Criterion) {
    c.bench_function("cache_line_not_padded", |b| {
        b.iter(|| {
            let counters = Arc::new(SharedCacheLine::new());
            run_shared(black_box(counters));
        })
    });
}

pub fn bench_padded(c: &mut Criterion) {
    c.bench_function("cache_line_padded", |b| {
        b.iter(|| {
            let counters = Arc::new(PaddedCacheLine::new());
            run_padded(black_box(counters));
        })
    });
}

criterion_group!(benches, bench_not_padded, bench_padded);
criterion_main!(benches);
