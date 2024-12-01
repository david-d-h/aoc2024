use std::cmp::Ordering;

use super::{process_raw_input, SOURCE};

#[inline]
pub fn distance((lhs, rhs): (u32, u32)) -> u32 {
    match lhs.cmp(&rhs) {
        Ordering::Equal => 0,
        Ordering::Greater => lhs - rhs,
        Ordering::Less => rhs - lhs,
    }
}

pub fn solution(container1: Vec<u32>, container2: Vec<u32>) -> u32 {
    container1
        .into_iter()
        .zip(container2)
        .map(distance)
        .sum::<u32>()
}

pub fn run() -> String {
    let (container1, container2) = process_raw_input(SOURCE.lines(), 1000);

    let distance = solution(container1, container2);

    format!("summed distance: {distance}")
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
            let distance = solution(container1.clone(), container2.clone());
            assert_eq!(distance, 1970720);
        });
    }
}
