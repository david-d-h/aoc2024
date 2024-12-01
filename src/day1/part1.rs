use super::{process_raw_input, INPUT};

pub fn solution(container1: &[u32], container2: &[u32]) -> u32 {
    container1
        .iter()
        .zip(container2)
        .map(|(&lhs, rhs)| lhs.abs_diff(*rhs))
        .sum::<u32>()
}

pub fn run() -> String {
    let (mut container1, mut container2) = process_raw_input(INPUT.lines(), 1000);

    container1.sort();
    container2.sort();

    let distance = solution(&container1, &container2);

    format!("summed distance: {distance}")
}
