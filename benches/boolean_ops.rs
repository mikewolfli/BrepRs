use criterion::{criterion_group, criterion_main, Criterion};

fn boolean_ops_benchmark(c: &mut Criterion) {
    c.bench_function("placeholder", |b| {
        b.iter(|| 1 + 1)
    });
}

criterion_group!(benches, boolean_ops_benchmark);
criterion_main!(benches);
