# Benchmarking

Cargo has built-in benchmarking, but using it requires the *nightly* unstable code channel. I generally don't recommend relying on nightly code! If you are writing performance-critical code, benchmarking is essential. Fortunately, Rust makes it relatively straightforward to include benchmarks with a bit of boilerplate.

## Quick and Dirty Benchmarks

> This example is in `project/simple_bench`

A quick and dirty way to benchmark operations is to use `Instant` and `Duration`:

```rust
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut i = 0;
    for j in 0 .. 1_000 {
        i += j*j;
    }
    let elapsed = now.elapsed();
    println!("Time elapsed: {} nanos", elapsed.as_nanos());
    println!("{i}");
}
```

## Criterion

> This project is in `projects/part2/criterion_bench`

In `Cargo.toml`, add:

```toml
[dev-dependencies]
criterion = { version = "0.4", features = [ "html_reports" ] }

[[bench]]
name = "my_benchmark"
harness = false
```

> `[dev-dependencies]` is new! This is a dependency that is *only* loaded by development tools, and isn't integrated into your final program. No space is wasted.

Create `<project>/benches/my_benchmark.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

Run `cargo bench` and see the result.

Go to `target/criterion` and you have a full HTML report with statistics.

## Flamegraphs

It pretty much requires Linux (and the `perf` infrastructure), but it's worth looking at [Cargo Flamegraphs](https://github.com/flamegraph-rs/flamegraph) if you are developing on that platform. It's an easy wrapper around `perf` for generating flamegraphs to find your hotspots.
