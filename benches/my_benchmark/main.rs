// Example of how to include a 2nd file.
mod util;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[cfg(all(
    any(target_os = "linux", target_os = "macos"),
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
))]
use pprof::criterion::{Output, PProfProfiler};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

#[cfg(all(
    any(target_os = "linux", target_os = "macos"),
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
))]
criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}

#[cfg(not(all(
    any(target_os = "linux", target_os = "macos"),
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
)))]
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
}

criterion_main!(benches);
