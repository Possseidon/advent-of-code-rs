mod year_2015;

mod cmd;
mod puzzle;
mod template;

use std::{env::VarError, iter::once, time::Duration};

use anyhow::{bail, Context, Result};
use clap::Parser;
use cmd::Args;
use puzzle::Puzzle;
use template::generate_template;

const ADVENT_OF_CODE_SESSION: &str = "ADVENT_OF_CODE_SESSION";

fn main() -> Result<()> {
    dotenv()?;

    let args = Args::parse();
    let puzzle = Puzzle::from_args(&args)?;

    puzzle.print_header();

    if args.generate {
        if args.example.is_some() {
            bail!("template generation incompatible with running an example");
        }
        if args.bench.is_some() {
            bail!("template generation incompatible with benchmarking");
        }
        if args.compare {
            bail!("compare can only be used with benchmarking");
        }
        if args.part2 {
            bail!("template generation always generates both parts");
        }
        if args.solution.is_some() {
            bail!("template generation does not support generating named solutions");
        }

        generate_template(puzzle.year, puzzle.day)?;
    } else if let Some(bench_duration) = args.bench {
        if args.example.is_some() {
            bail!("benchmark cannot be run on examples");
        }

        #[cfg(debug_assertions)]
        {
            println!("\x1b[33mWARNING: Running benchmark with a debug build\x1b[0m");
            println!();
        }

        let session = &get_session()?;
        let bench_duration = Duration::from_secs_f32(bench_duration.unwrap_or(1.0));

        if args.compare {
            if args.solution.is_some() {
                bail!("compare always runs all solutions");
            }

            puzzle.print_benchmark_comparison(session, bench_duration)?;
        } else {
            puzzle.print_benchmark(args.solution.as_deref(), session, bench_duration)?;
        }
    } else if let Some(example) = args.example {
        if args.compare {
            bail!("compare can only be used with benchmarking");
        }

        let examples = puzzle.get_examples();
        if examples.is_empty() {
            bail!("puzzle has no examples");
        }
        if let Some(example) = example {
            puzzle.run_examples(
                args.solution.as_deref(),
                &get_session()?,
                once(
                    *examples.get(example).with_context(|| {
                        format!("puzzle only has {} example(s)", examples.len())
                    })?,
                ),
            )?;
        } else {
            puzzle.run_examples(
                args.solution.as_deref(),
                &get_session()?,
                examples.iter().copied(),
            )?;
        };
    } else {
        if args.compare {
            bail!("compare can only be used with benchmarking");
        }

        puzzle.solve(args.solution.as_deref(), &get_session()?)?;
    }

    Ok(())
}

fn dotenv() -> Result<()> {
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(err) if err.not_found() => Ok(()),
        Err(err) => Err(err).context("failed to initialize environment from `.env`")?,
    }
}

fn get_session() -> Result<String> {
    match std::env::var(ADVENT_OF_CODE_SESSION) {
        Ok(session) => Ok(session),
        Err(VarError::NotPresent) => {
            bail!("{ADVENT_OF_CODE_SESSION} env var required to get puzzle input")
        }
        Err(error) => Err(error)?,
    }
}
