// This benchmark checks the influence of "unlikely" hint in `find_oldest_new_with_hint()`.
// The result shows that the version with hint is faster then the version without the hint.
// But the classic, iterator based version is also fast.

use std::hint::black_box;

use bench_core::Foo;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::seq::SliceRandom;

pub fn bench_find_oldest(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut data = std::iter::repeat_with(|| Foo::random(&mut rng))
        .take(300_000)
        .collect::<Vec<_>>();

    let _oldest = c.bench_function("find_oldest", |b| {
        data.shuffle(&mut rng);
        b.iter(|| black_box(bench_core::find_oldest(black_box(&data))))
    });
}

pub fn bench_find_oldest_alternative(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut data = std::iter::repeat_with(|| Foo::random(&mut rng))
        .take(300_000)
        .collect::<Vec<_>>();

    let _oldest = c.bench_function("find_oldest_alternative", |b| {
        data.shuffle(&mut rng);
        b.iter(|| black_box(bench_core::find_oldest_alternative(black_box(&data))))
    });
}

pub fn bench_find_oldest_alternative_with_hint(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut data = std::iter::repeat_with(|| Foo::random(&mut rng))
        .take(300_000)
        .collect::<Vec<_>>();

    let _oldest = c.bench_function("find_oldest_alternative_with_hint", |b| {
        data.shuffle(&mut rng);
        b.iter(|| {
            black_box(bench_core::find_oldest_alternative_with_hint(black_box(
                &data,
            )))
        })
    });
}

criterion_group!(
    bench,
    bench_find_oldest_alternative,
    bench_find_oldest_alternative_with_hint
);
criterion_main!(bench);
