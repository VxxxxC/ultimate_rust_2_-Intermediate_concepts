use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::splish;

pub fn splish_bench(c: &mut Criterion) {
    c.bench_function("Splish Speed", |v| {
        v.iter(|| splish(black_box(255), black_box(255)))
    });
}

criterion_group!(benches, splish_bench);
criterion_main!(benches);
