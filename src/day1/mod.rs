use std::str::Lines;

pub mod part1;
pub mod part2;

pub const INPUT: &str = core::include_str!("../../res/day1.txt");

#[inline]
fn parse(source: &str) -> u32 {
    source.parse().expect("found invalid input")
}

pub fn process_raw_input(source: Lines<'_>, length: usize) -> (Vec<u32>, Vec<u32>) {
    let (mut container1, mut container2) = (Vec::with_capacity(length), Vec::with_capacity(length));

    source.for_each(|line| {
        container1
            .push_within_capacity(parse(&line[0..=4]))
            .and_then(|_| container2.push_within_capacity(parse(&line[8..=12])))
            .expect("count of lines in source should not exceed the specified amount");
    });

    (container1, container2)
}
