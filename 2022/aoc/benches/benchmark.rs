use aoc::day::{Day, day_03::Day03};
use criterion::{criterion_group, criterion_main, Criterion};


fn run_benchmark() {
    Day03 {}.solve();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Benchmark result:", |b| b.iter(|| run_benchmark()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);