use crate::puzzle::{AdventOfCode, Day, Example, Part, PuzzleResult, Solution};

impl Part<1> for (AdventOfCode<2023>, Day<2>) {
    const SOLUTIONS: &'static [Solution] = &[Solution("solution", |input| {
        let valid = |line: &str| {
            let (_, cubes) = line.split_once(':').unwrap();
            cubes.trim_start().split(';').all(|cubes| {
                cubes.split(',').all(|cubes| {
                    let (amount, color) = cubes.trim_start().split_once(' ').unwrap();
                    let amount = amount.parse::<i32>().unwrap();
                    match color {
                        "red" => amount <= 12,
                        "green" => amount <= 13,
                        "blue" => amount <= 14,
                        _ => panic!(),
                    }
                })
            })
        };
        PuzzleResult::Int(
            input
                .lines()
                .enumerate()
                .filter_map(|(i, line)| valid(line).then_some(i as i32 + 1))
                .sum::<i32>(),
        )
    })];

    const EXAMPLES: &'static [Example] = &[Example(3, 4)];
}

impl Part<2> for (AdventOfCode<2023>, Day<2>) {
    const SOLUTIONS: &'static [Solution] = &[Solution("solution", |input| {
        PuzzleResult::Int(
            input
                .lines()
                .map(|line| {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;
                    let (_, cubes) = line.split_once(':').unwrap();
                    for cubes in cubes.trim_start().split(';') {
                        for cubes in cubes.split(',') {
                            let (amount, color) = cubes.trim_start().split_once(' ').unwrap();
                            let amount = amount.parse::<i32>().unwrap();
                            match color {
                                "red" => red = red.max(amount),
                                "green" => green = green.max(amount),
                                "blue" => blue = blue.max(amount),
                                _ => panic!(),
                            }
                        }
                    }
                    red * green * blue
                })
                .sum::<i32>(),
        )
    })];

    const EXAMPLES: &'static [Example] = &[Example(6, 12)];
}
