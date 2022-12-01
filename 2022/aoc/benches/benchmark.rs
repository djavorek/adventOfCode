use aoc::day::{Day, day_01::Day01};
use criterion::{criterion_group, criterion_main, Criterion};


fn run_benchmark() {
    Day01 {}.solve();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Benchmark result:", |b| b.iter(|| run_benchmark()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);