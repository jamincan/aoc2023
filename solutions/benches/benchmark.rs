use aoc2023::SOLUTIONS;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    for (day, (pt1, pt2)) in SOLUTIONS.iter().enumerate() {
        let day = day + 1;
        c.bench_function(&format!("d{day}p1"), |b| b.iter(|| pt1()));
        c.bench_function(&format!("d{day}p2"), |b| b.iter(|| pt2()));
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
