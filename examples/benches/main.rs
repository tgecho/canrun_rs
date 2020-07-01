use canrun_examples::zebra::zebra;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn zebra_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("zebra");
    group.sample_size(10);
    group.bench_function("zebra", |b| b.iter(|| zebra()));
    group.finish();
}

criterion_group!(benches, zebra_benchmark);
criterion_main!(benches);
