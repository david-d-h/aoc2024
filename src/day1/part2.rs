use std::collections::HashMap;

use super::{process_raw_input, SOURCE};

pub fn build_presence_map(source: Vec<u32>) -> HashMap<u32, u32> {
    let mut indexes = HashMap::<u32, u32>::new();

    source.into_iter().for_each(|curr| {
        *indexes.entry(curr).or_default() += 1;
    });

    indexes
}

pub fn solution(container1: Vec<u32>, container2: Vec<u32>) -> u32 {
    let presence = build_presence_map(container2);

    container1.into_iter().fold(0, |acc, curr| {
        acc + curr * presence.get(&curr).cloned().unwrap_or(0)
    })
}

pub fn run() -> String {
    let (container1, container2) = process_raw_input(SOURCE.lines(), 1000);

    let score = solution(container1, container2);

    format!("summed similarity score: {score}")
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let (container1, container2) = process_raw_input(SOURCE.lines(), 1000);

        b.iter(|| {
            let score = solution(container1.clone(), container2.clone());
            assert_eq!(score, 17191599);
        })
    }
}
