use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use aoc::day2::*;

fn part_one(c: &mut Criterion) {
    c.bench_function("d2p1", |b| {
        b.iter(|| {
            let safe_count = part1::solution(black_box(INPUT));
            assert_eq!(safe_count, 606);
        });
    });
}

criterion_group!(benches, part_one);
criterion_main!(benches);
