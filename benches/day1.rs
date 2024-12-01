use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use aoc::day1::*;

fn part_one(c: &mut Criterion) {
    let containers = process_raw_input(INPUT.lines(), 1000);
    let mut sorted_containers = containers.clone();
    sorted_containers.0.sort();
    sorted_containers.1.sort();

    c.bench_function("d1p1_without_sorting", |b| {
        b.iter(|| {
            let distance = part1::solution(
                black_box(&sorted_containers.0),
                black_box(&sorted_containers.1),
            );
            assert_eq!(distance, 1970720);
        });
    });

    c.bench_function("d1p1_without_sorting_batched_ref", |b| {
        b.iter_batched_ref(
            || sorted_containers.clone(),
            |(container1, container2)| {
                let distance = part1::solution(black_box(container1), black_box(container2));
                assert_eq!(distance, 1970720);
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("d1p1_with_sorting_batched_ref", |b| {
        b.iter_batched_ref(
            || containers.clone(),
            |(container1, container2)| {
                container1.sort();
                container2.sort();
                let distance = part1::solution(black_box(container1), black_box(container2));
                assert_eq!(distance, 1970720);
            },
            BatchSize::SmallInput,
        );
    });
}

fn part_two(c: &mut Criterion) {
    let containers = process_raw_input(INPUT.lines(), 1000);
    let mut sorted_containers = containers.clone();
    sorted_containers.0.sort();
    sorted_containers.1.sort();

    c.bench_function("d1p2_without_sorting", |b| {
        b.iter(|| {
            let score = part2::solution(&sorted_containers.0, &sorted_containers.1);
            assert_eq!(score, 17191599);
        });
    });

    c.bench_function("d1p2_with_sorting_batched_ref", |b| {
        b.iter_batched_ref(
            || containers.clone(),
            |(container1, container2)| {
                container1.sort();
                container2.sort();
                let score = part2::solution(container1, container2);
                assert_eq!(score, 17191599);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);
