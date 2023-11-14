use std::{
    hint::black_box,
    io::{stdout, Write},
    time::{Duration, Instant},
};

use anyhow::{bail, Context, Result};
use chrono::{DateTime, Datelike, Utc};
use chrono_tz::{Tz, EST};
use humantime::format_duration;
use num_traits::ToPrimitive;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use thousands::Separable;

use crate::cmd::Args;

pub(crate) struct AdventOfCode<const YEAR: u32>;
pub(crate) struct Day<const DAY: u8>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Puzzle {
    pub(crate) year: PuzzleYear,
    pub(crate) day: PuzzleDay,
    pub(crate) part: PuzzlePart,
}

pub(crate) type PuzzleYear = bounded_integer::BoundedU32<2015, { u32::MAX }>;
pub(crate) type PuzzleDay = bounded_integer::BoundedU8<1, 25>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PuzzlePart {
    Part1,
    Part2,
}

pub(crate) trait Part<const N: u8> {
    fn solve(input: &str) -> Result<String>;

    const EXAMPLES: &'static [Example] = &[];
}

type Solve = fn(&str) -> Result<String>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Example(pub(crate) usize, pub(crate) usize);

impl Puzzle {
    pub(crate) fn from_args(args: &Args) -> Result<Self> {
        let part = if args.part2 {
            PuzzlePart::Part2
        } else {
            PuzzlePart::Part1
        };
        match args {
            Args {
                year: None,
                day: None,
                ..
            } => {
                let now = advent_of_code_now();
                if now.month() != 12 {
                    bail!("Current Day can only be deduced in December; please specify");
                }
                Puzzle::new(now.year(), now.day(), part)
            }
            Args {
                day: Some(day),
                year: None,
                ..
            } => {
                let now = advent_of_code_now();
                Puzzle::new(
                    now.year() - if now.month() < 12 { 1 } else { 0 },
                    *day,
                    part,
                )
            }
            Args {
                year: Some(year),
                day: None,
                ..
            } => bail!("Please specify which day of {year} to run"),
            Args {
                year: Some(year),
                day: Some(day),
                ..
            } => Puzzle::new(*year, *day, part),
        }
    }

    fn new(year: impl ToPrimitive, day: impl ToPrimitive, part: PuzzlePart) -> Result<Self> {
        Ok(Self {
            year: year
                .to_u32()
                .and_then(PuzzleYear::new)
                .context("Invalid year; the first year of Advent of Code was 2015")?,
            day: day
                .to_u8()
                .and_then(PuzzleDay::new)
                .context("Day must be between 1 and 25")?,
            part,
        })
    }

    fn puzzle_url(&self) -> String {
        format!("https://adventofcode.com/{}/day/{}", self.year, self.day)
    }

    fn input_url(&self) -> String {
        format!("{}/input", self.puzzle_url())
    }

    fn get_with_session(&self, session: &str, url: &str) -> Result<String> {
        Ok(Client::builder()
            .build()?
            .get(url)
            .header("cookie", format!("session={session}"))
            .send()?
            .text()?)
    }

    fn get_input(&self, session: &str) -> Result<String> {
        self.get_with_session(session, &self.input_url())
    }

    fn get_code_blocks(&self, session: &str) -> Result<Vec<String>> {
        Ok(
            Html::parse_document(&self.get_with_session(session, &self.puzzle_url())?)
                .select(&Selector::parse("code").unwrap())
                .map(|element| element.inner_html())
                .collect(),
        )
    }

    pub(crate) fn print_header(&self) {
        println!(
            "Advent of Code {} - Day {} - {}",
            self.year,
            self.day,
            match self.part {
                PuzzlePart::Part1 => "Part 1",
                PuzzlePart::Part2 => "Part 2",
            }
        );
        println!();
    }

    pub(crate) fn get_input_verbose(&self, session: &str) -> Result<String> {
        print!("Grabbing input... ");
        stdout().flush()?;
        let input = self.get_input(session)?;
        println!(" got {} bytes.", input.len());
        println!();
        Ok(input)
    }

    pub(crate) fn solve(&self, session: &str) -> Result<()> {
        let solve = self.get_solution().context("puzzle not implemented")?;
        let input = self.get_input_verbose(session)?;
        let result = solve(&input)?;
        println!("{}", result);
        Ok(())
    }

    pub(crate) fn run_examples(
        &self,
        session: &str,
        examples: impl Iterator<Item = Example>,
    ) -> Result<()> {
        let solve = self.get_solution().context("puzzle not implemented")?;

        print!("Scraping Example Inputs... ");
        stdout().flush()?;
        let code_blocks = self.get_code_blocks(session)?;
        println!("Done!");
        println!();

        let mut success = 0;
        let mut total = 0;
        println!("| Running Examples... ");
        println!("|---------------------");
        for Example(input_offset, expected_result_offset) in examples {
            total += 1;
            let input = code_blocks
                .get(input_offset)
                .context("example offset out of bounds")?;
            let expected_result = code_blocks
                .get(expected_result_offset)
                .context("expected result offset out of bounds")?;
            let result = solve(input);
            match result {
                Ok(result) if &result == expected_result => {
                    println!("| Example #{total} passed");
                    success += 1;
                }
                Ok(result) => {
                    println!("| Example #{total} failed: {expected_result} != {result}");
                    println!("|- Input: {input}");
                }
                Err(error) => {
                    println!("| Example #{total} failed: {error}");
                    println!("|- Input: {input}");
                }
            }
        }
        if total > 0 {
            println!("|---------------------");
            println!("| {success} / {total} Examples passed");
        } else {
            println!("| No Examples found");
        }
        Ok(())
    }

    pub(crate) fn benchmark(&self, session: &str, bench_duration: Duration) -> Result<()> {
        let solve = self.get_solution().context("puzzle not implemented")?;
        let input = self.get_input_verbose(session)?;

        let start = Instant::now();
        let mut iterations = 0;
        while start.elapsed() < bench_duration {
            black_box(solve(black_box(&input))?);
            iterations += 1;
        }

        let elapsed = start.elapsed();

        println!("Benchmark ran for {}", format_duration(bench_duration));
        println!("  {} Iterations", iterations.separate_with_commas());
        println!("  {}/Iteration", format_duration(elapsed / iterations));
        println!();

        Ok(())
    }
}

fn advent_of_code_now() -> DateTime<Tz> {
    Utc::now().with_timezone(&EST)
}

macro_rules! puzzles {
    ( $( $year:literal => [ $( $day:literal )* ] )* ) => {
        impl Puzzle {
            pub(crate) fn get_solution(self) -> Option<Solve> {
                match u32::from(self.year) {
                    $( $year => match u8::from(self.day) {
                        $( $day => match self.part {
                            PuzzlePart::Part1 => Some(<(AdventOfCode<$year>, Day<$day>) as Part<1>>::solve),
                            PuzzlePart::Part2 => Some(<(AdventOfCode<$year>, Day<$day>) as Part<2>>::solve),
                        })*
                        _ => None,
                    } )*
                    _ => None,
                }
            }

            pub(crate) fn get_examples(self) -> Option<&'static [Example]> {
                match u32::from(self.year) {
                    $( $year => match u8::from(self.day) {
                        $( $day => match self.part {
                            PuzzlePart::Part1 => Some(<(AdventOfCode<$year>, Day<$day>) as Part<1>>::EXAMPLES),
                            PuzzlePart::Part2 => Some(<(AdventOfCode<$year>, Day<$day>) as Part<2>>::EXAMPLES),
                        })*
                        _ => None,
                    } )*
                    _ => None,
                }
            }
        }
    };
}

puzzles! {
    2015 => [ 1 ]
}
