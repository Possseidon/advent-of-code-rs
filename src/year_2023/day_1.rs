use crate::puzzle::{AdventOfCode, Day, Example, Part, PuzzleResult, Solution};

impl Part<1> for (AdventOfCode<2023>, Day<1>) {
    const SOLUTIONS: &'static [Solution] = &[Solution("solution", |input| {
        PuzzleResult::Int(
            input
                .lines()
                .map(|line| {
                    let get_digit = |char: u8| char.is_ascii_digit().then_some(char - b'0');
                    let left = line.bytes().find_map(get_digit).unwrap() as i32;
                    let right = line.bytes().rev().find_map(get_digit).unwrap() as i32;
                    left * 10 + right
                })
                .sum::<i32>(),
        )
    })];

    const EXAMPLES: &'static [Example] = &[Example(0, 5)];
}

const SPELLED_OUT_DIGITS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

impl Part<2> for (AdventOfCode<2023>, Day<1>) {
    const SOLUTIONS: &'static [Solution] = &[Solution("solution", |input| {
        PuzzleResult::Int(
            input
                .lines()
                .map(|line| {
                    let line = line.as_bytes();

                    let get_digit = |index: usize| {
                        if line[index].is_ascii_digit() {
                            return Some((line[index] - b'0') as i32);
                        }
                        for (i, name) in SPELLED_OUT_DIGITS.iter().enumerate() {
                            if line[index..].starts_with(name) {
                                return Some(i as i32 + 1);
                            }
                        }
                        None
                    };

                    let indices = 0..line.len();
                    let left = indices.clone().find_map(get_digit).unwrap();
                    let right = indices.rev().find_map(get_digit).unwrap();

                    left * 10 + right
                })
                .sum::<i32>(),
        )
    })];

    const EXAMPLES: &'static [Example] = &[Example(16, 24)];
}
