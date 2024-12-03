pub mod part1;

pub const INPUT: &[u8] = include_bytes!("../../res/day2.txt");
pub const SAMPLE: &[u8] =
    "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n".as_bytes();

/// Converts a single byte to a single digit (u8, 0..=9)
#[track_caller]
fn byte_to_int(byte: u8) -> u32 {
    (byte - b'0') as u32
}

fn construct_number(bytes: &[u8]) -> u32 {
    bytes
        .into_iter()
        .fold(0, |acc, num| acc * 10 + byte_to_int(*num))
}

#[derive(Debug, Clone)]
pub struct LevelIter<'a> {
    report: &'a [u8],
    levels: tinyvec::ArrayVec<[usize; 8]>,
    index: usize,
}

impl LevelIter<'_> {
    #[inline]
    fn nth(&self, n: usize) -> u32 {
        let start = if n == 1 { 0 } else { self.levels[n - 2] + 1 };
        construct_number(&self.report[start..=self.levels[n - 1] - 1])
    }
}

impl Iterator for LevelIter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let start = if self.index == 0 {
            0
        } else if self.index == self.levels.len() {
            return None;
        } else {
            self.levels[self.index - 1] + 1
        };

        self.report
            .get(start..self.levels[self.index])
            .map(construct_number)
            .inspect(|_| {
                self.index += 1;
            })
    }
}

#[derive(Debug)]
pub struct ReportIter<'a> {
    reports: &'a [u8],
    prev_line_end: usize,
    index: usize,
}

impl ReportIter<'_> {
    fn new<'a>(bytes: &'a [u8]) -> ReportIter<'a> {
        ReportIter {
            reports: bytes,
            prev_line_end: 0,
            index: 0,
        }
    }
}

impl<'a> Iterator for ReportIter<'a> {
    type Item = LevelIter<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.reports.len() {
            return None;
        }

        let mut levels = tinyvec::ArrayVec::<[usize; 8]>::new();
        let anchor = self.index;
        let end_of_line = loop {
            self.index += 1;

            match self.reports[self.index] {
                b' ' => levels.push(self.index - self.prev_line_end),
                b'\n' => {
                    levels.push(self.index - self.prev_line_end);
                    self.prev_line_end = self.index;
                    break self.index;
                }
                _ => (),
            }
        };

        let report = &self.reports[anchor..end_of_line];
        self.index += 1;
        self.prev_line_end = self.index;

        Some(LevelIter {
            report,
            levels,
            index: 0,
        })
    }
}
