use criterion::{criterion_group, criterion_main, Criterion};
use impl_vs_dyn::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("via_impl, true", |b| b.iter(|| via_impl(true).sum::<u32>()));
    c.bench_function("via_dyn, true", |b| b.iter(|| via_dyn(true).sum::<u32>()));
    c.bench_function("via_impl, false", |b| b.iter(|| via_impl(false).sum::<u32>()));
    c.bench_function("via_dyn, false", |b| b.iter(|| via_dyn(false).sum::<u32>()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
