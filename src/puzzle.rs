use std::{
    hint::black_box,
    io::{stdout, Write},
    iter::once,
    time::{Duration, Instant},
};

use anyhow::{bail, Context, Result};
use chrono::{DateTime, Datelike, Utc};
use chrono_tz::{Tz, EST};
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
    const SOLUTIONS: &'static [Solution] = &[];
    const EXAMPLES: &'static [Example] = &[];
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Solution(pub(crate) &'static str, pub(crate) SolutionFn);

pub(crate) type SolutionFn = fn(input: &str) -> PuzzleResult;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PuzzleResult {
    Int(i32),
    Str(String),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Example(pub(crate) usize, pub(crate) usize);

struct BenchmarkResult {
    runtime: Duration,
    overhead: Duration,
    iterations: usize,
    average: Duration,
    std_dev: Duration,
    min: Duration,
    med: Duration,
    max: Duration,
}

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
        Html::parse_document(&self.get_with_session(session, &self.puzzle_url())?)
            .select(&Selector::parse("code").unwrap())
            .map(|element| {
                Ok(element
                    .text()
                    .next()
                    .context("malformed example")?
                    .to_string())
            })
            .collect()
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
        println!("got {} bytes.", input.len());
        println!();
        Ok(input)
    }

    pub(crate) fn solve(&self, solution: Option<&str>, session: &str) -> Result<()> {
        let Solution(_, solve) = self.get_solution(solution)?;
        let input = self.get_input_verbose(session)?;
        let result = solve(&input);
        println!("{}", result);
        Ok(())
    }

    pub(crate) fn run_examples(
        &self,
        solution: Option<&str>,
        session: &str,
        examples: impl Iterator<Item = Example>,
    ) -> Result<()> {
        let Solution(_, solve) = self.get_solution(solution)?;

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
            if &format!("{}", result) == expected_result {
                println!("| Example #{total} passed");
                success += 1;
            } else {
                println!("| Example #{total} failed: {expected_result} != {result}");
                println!("|- Input: {input}");
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

    pub(crate) fn print_benchmark(
        &self,
        solution: Option<&str>,
        session: &str,
        bench_duration: Duration,
    ) -> Result<()> {
        let Solution(_, solve) = self.get_solution(solution)?;
        let input = self.get_input_verbose(session)?;

        let BenchmarkResult {
            runtime,
            overhead,
            iterations,
            average,
            std_dev,
            min,
            med,
            max,
        } = self.benchmark(solve, &input, bench_duration);

        println!("Benchmark ran for {runtime:.2?} (plus {overhead:.2?} of overhead)");
        println!("  Iterations: {}", iterations.separate_with_commas());
        println!("  Avg±StdDev: {average:.2?} ± {std_dev:.2?}");
        println!(" Min<Med<Max: {min:.2?} < {med:.2?} < {max:.2?}");
        println!();

        Ok(())
    }

    pub(crate) fn print_benchmark_comparison(
        &self,
        session: &str,
        bench_duration: Duration,
    ) -> Result<()> {
        let input = self.get_input_verbose(session)?;

        let solutions = self.get_solutions();
        if solutions.is_empty() {
            bail!("puzzle has no solutions");
        }

        const SOLUTION: &str = "Solution";
        let name_width = solutions
            .iter()
            .map(|Solution(name, _)| name.len())
            .chain(once(SOLUTION.len()))
            .max()
            .unwrap();

        let mut benchmark_results = solutions
            .iter()
            .copied()
            .enumerate()
            .inspect(|(i, Solution(name, _))| {
                print!(
                    "\r\x1b[KBenchmarking {}/{} - {name}",
                    i + 1,
                    solutions.len(),
                );
                stdout().flush().unwrap();
            })
            .map(|(_, Solution(name, solve))| {
                (
                    name,
                    solve(&input),
                    self.benchmark(solve, &input, bench_duration),
                )
            })
            .collect::<Vec<_>>();
        print!("\r\x1b[2K");

        let first_puzzle_result = benchmark_results.first().unwrap().1.clone();

        benchmark_results.sort_by_key(|(_, _, result)| result.average);

        let fastest_time = benchmark_results[0].2.average;

        const WS: &str = "";

        println!("  {WS: <name_width$} ┏━━ Averge ±   StdDev ┯ Relative ┳━ Mininum ┯━━ Median ┯━ Maximum ┓");
        println!("┏━{WS:━<name_width$}━╋━━━━━━━━━━━━━━━━━━━━━┿━━━━━━━━━━╋━━━━━━━━━━┿━━━━━━━━━━┿━━━━━━━━━━┫");

        for (
            name,
            puzzle_result,
            BenchmarkResult {
                average,
                std_dev,
                min,
                med,
                max,
                ..
            },
        ) in &benchmark_results
        {
            let wrong = puzzle_result != &first_puzzle_result;
            let rel = (average.as_secs_f32() / fastest_time.as_secs_f32() - 1.0) * 100.0;
            if wrong {
                print!("\x1b[90m");
            }
            print!("┃ {name:<name_width$} ┃ {average:>8.2?} ± {std_dev:>8.2?} │ {rel:>7.1}% ┃ {min:>8.2?} │ {med:>8.2?} │ {max:>8.2?} ┃");
            if wrong {
                print!(" \x1b[33m{puzzle_result} != {first_puzzle_result}\x1b[0m");
            }
            println!();
        }

        println!("┗━{WS:━<name_width$}━┻━━━━━━━━━━━━━━━━━━━━━┷━━━━━━━━━━┻━━━━━━━━━━┷━━━━━━━━━━┷━━━━━━━━━━┛");

        Ok(())
    }

    fn benchmark(
        &self,
        solve: SolutionFn,
        input: &str,
        bench_duration: Duration,
    ) -> BenchmarkResult {
        // Using Vec and then sort to minimize overhead compared to e.g. BTreeSet.
        // Pre-allocating some capacity doesn't make much difference and picking a good initial
        // capacity isn't really possible without running the benchmark upfront.
        let mut times = vec![];
        let start = Instant::now();
        loop {
            let iteration_start = Instant::now();
            black_box(solve(black_box(input)));
            times.push(iteration_start.elapsed());

            if start.elapsed() >= bench_duration {
                break;
            }
        }
        let elapsed_with_overhead = start.elapsed();
        let runtime = times.iter().sum::<Duration>();
        let overhead = elapsed_with_overhead - runtime;

        times.sort_unstable();

        let iterations = times.len();
        let average = runtime.div_f32(iterations as f32);
        let std_dev = if iterations > 1 {
            Duration::from_secs_f32(
                times
                    .iter()
                    .map(|time| (time.as_secs_f32() - average.as_secs_f32()).powi(2))
                    .sum::<f32>()
                    .sqrt()
                    / (iterations as f32 - 1.0),
            )
        } else {
            Duration::ZERO
        };

        BenchmarkResult {
            runtime,
            overhead,
            iterations,
            average,
            std_dev,
            min: *times.first().unwrap(),
            med: if iterations % 2 == 0 {
                (times[iterations / 2 - 1] + times[iterations / 2]) / 2
            } else {
                times[iterations / 2]
            },
            max: *times.last().unwrap(),
        }
    }

    fn get_solution(&self, solution: Option<&str>) -> Result<Solution> {
        let solutions = self.get_solutions();
        if let Some(solution) = solution {
            solutions
                .iter()
                .find(|Solution(name, _)| *name == solution)
                .copied()
                .context("solution not found")
        } else {
            solutions.first().copied().context("puzzle not implemented")
        }
    }
}

impl std::fmt::Display for PuzzleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PuzzleResult::Int(result) => write!(f, "{result}"),
            PuzzleResult::Str(result) => write!(f, "{result}"),
        }
    }
}

fn advent_of_code_now() -> DateTime<Tz> {
    Utc::now().with_timezone(&EST)
}

macro_rules! puzzles {
    ( $( $year:literal => [ $( $day:literal )* ] )* ) => {
        impl Puzzle {
            pub(crate) fn get_solutions(self) -> &'static [Solution]{
                match u32::from(self.year) {
                    $( $year => match u8::from(self.day) {
                        $( $day => match self.part {
                            PuzzlePart::Part1 => <(AdventOfCode<$year>, Day<$day>) as Part<1>>::SOLUTIONS,
                            PuzzlePart::Part2 => <(AdventOfCode<$year>, Day<$day>) as Part<2>>::SOLUTIONS,
                        })*
                        _ => &[],
                    } )*
                    _ => &[],
                }
            }

            pub(crate) fn get_examples(self) -> &'static [Example] {
                match u32::from(self.year) {
                    $( $year => match u8::from(self.day) {
                        $( $day => match self.part {
                            PuzzlePart::Part1 => <(AdventOfCode<$year>, Day<$day>) as Part<1>>::EXAMPLES,
                            PuzzlePart::Part2 => <(AdventOfCode<$year>, Day<$day>) as Part<2>>::EXAMPLES,
                        })*
                        _ => &[],
                    } )*
                    _ => &[],
                }
            }
        }
    };
}

puzzles! {
    2015 => [ 1 ]
}
