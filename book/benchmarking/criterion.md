# Benchmarking using Criterion

## How to benchmark functions using criterion library?

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn fib(n: i32) -> i32 {
    if n > 1 {
        fib(n - 1) + fib(n - 2)
    } else {
        1
    }
}

pub fn fib_fast(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;

    for _ in 0..n {
        b += a;
        a = b - a;
    }

    a
}

fn criterion_fib_slow(c: &mut Criterion) {
    c.bench_function("fib 10", |b| b.iter(|| fib(black_box(10))));
}

fn criterion_fib_fast(c: &mut Criterion) {
    c.bench_function("fib_fast 10", |b| b.iter(|| fib_fast(black_box(10))));
}

criterion_group!(benches, criterion_fib_fast, criterion_fib_slow);
criterion_main!(benches);
```

Add following dependencies to the `Cargo.toml`.

```yaml
[dev-dependencies]
criterion = "0.3.1"

[[bench]]
name = "lib"
path = "src/lib.rs"
harness = false
```

Benchmarks can be run by `cargo bench`.
