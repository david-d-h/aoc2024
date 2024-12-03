use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use aoc::day3::*;

fn part_one(c: &mut Criterion) {
    c.bench_function("d3p1_alloc", |b| {
        b.iter(|| {
            let parsed = part1::parse(black_box(INPUT));
            let accum = part1::solution(black_box(parsed));
            assert_eq!(accum, 181345830);
        });
    });

    c.bench_function("d3p1", |b| {
        b.iter(|| {
            let accum = parsing_eval(black_box(INPUT));
            assert_eq!(accum, 181345830);
        });
    });
}

fn part_two(c: &mut Criterion) {
    c.bench_function("d3p2", |b| {
        b.iter(|| {
            let accum = part2::with_blocking(INPUT);
            assert_eq!(accum, 98729041);
        });
    });
}

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);
