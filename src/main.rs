#![feature(
    vec_push_within_capacity,
    if_let_guard,
    let_chains,
    test,
    assert_matches
)]
use std::{env, num::NonZeroUsize};

pub mod day1;
pub mod day2;
pub mod day3;

type Solution = fn() -> String;

const PARTS: &[(Solution, Option<Solution>)] = &[
    (day1::part1::run, Some(day1::part2::run)),
    (day2::part1::run, None),
    (day3::part1::run, Some(day3::part2::run)),
];

fn run(day: NonZeroUsize, part: NonZeroUsize) -> Result<String, String> {
    let solutions = PARTS.get(day.get() - 1).ok_or(format!(
        "no implementation of either part exists for day {day}",
    ))?;

    let solution = match part.get() {
        1 => solutions.0,
        2 => solutions.1.ok_or(format!(
            "implementation of part {part} does not exist for day {day}"
        ))?,
        _ => Err("part must be either 1 or 2".to_string())?,
    };

    Ok(solution())
}

fn run_with_output(day: NonZeroUsize, part: NonZeroUsize) -> Result<(), String> {
    Ok(println!("{}", run(day, part)?))
}

fn main() -> Result<(), String> {
    let args = env::args().into_iter().skip(1);

    match &args.collect::<Vec<_>>()[..] {
        [day] if let Ok(day) = day.parse::<NonZeroUsize>() => {
            run_with_output(day, unsafe { NonZeroUsize::new_unchecked(1) })?;
            run_with_output(day, unsafe { NonZeroUsize::new_unchecked(2) })?;
        }
        [day, part]
            if let Ok(day) = day.parse::<NonZeroUsize>()
                && let Ok(part) = part.parse::<NonZeroUsize>() =>
        {
            run_with_output(day, part)?;
        }
        _ => println!("usage: <day:NonZeroU32> [part:1|2]"),
    };

    Ok(())
}
