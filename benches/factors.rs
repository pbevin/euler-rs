use criterion::{black_box, criterion_group, criterion_main, Criterion};
use euler::factors;

fn list_factors(n: i64) {
    let _ = factors(n).collect::<Vec<_>>();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("factor F_5", |b| {
        b.iter(|| list_factors(black_box(4294967297)))
    });
    c.bench_function("factor big prime", |b| {
        b.iter(|| list_factors(black_box(137438953481)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
