use super::{LevelIter, ReportIter, INPUT};

pub fn solution(input: &[u8]) -> usize {
    ReportIter::new(input)
        .filter_map(|mut report| {
            let ascending = LevelIter::nth(&report, 1) < LevelIter::nth(&report, 2);
            let mut maybe_prev = None;

            report
                .all(|level| {
                    let Some(prev) = maybe_prev else {
                        maybe_prev = Some(level);
                        return true;
                    };

                    maybe_prev = Some(level);

                    let diff = level.abs_diff(prev);

                    1 <= diff && diff <= 3 && (level > prev) ^ !ascending
                })
                .then_some(())
        })
        .count()
}

pub fn run() -> String {
    let safe = solution(INPUT);

    format!("amount of safe reports: {safe}")
}
