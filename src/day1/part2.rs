use std::collections::HashMap;

use super::{process_raw_input, INPUT};

pub fn build_presence_map(source: &[u32]) -> HashMap<u32, u32> {
    let mut indexes = HashMap::<u32, u32>::new();

    source.iter().for_each(|&curr| {
        *indexes.entry(curr).or_insert(0) += 1;
    });

    indexes
}

pub fn solution(container1: &[u32], container2: &[u32]) -> u32 {
    let presence = build_presence_map(container2);

    container1.into_iter().fold(0, |acc, curr| {
        acc + curr * presence.get(&curr).cloned().unwrap_or(0)
    })
}

pub fn run() -> String {
    let (container1, container2) = process_raw_input(INPUT.lines(), 1000);

    let score = solution(&container1, &container2);

    format!("summed similarity score: {score}")
}
