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
        if args.part2 {
            bail!("template generation always generates both parts");
        }

        generate_template(puzzle.year, puzzle.day)?;
    } else if let Some(bench_duration) = args.bench {
        if args.example.is_some() {
            bail!("benchmark cannot be run with an example");
        }

        puzzle.benchmark(
            &get_session()?,
            Duration::from_secs_f32(bench_duration.unwrap_or(1.0)),
        )?;
    } else if let Some(example) = args.example {
        let examples = puzzle.get_examples().context("puzzle not implemented")?;
        if let Some(example) = example {
            puzzle.run_examples(
                &get_session()?,
                once(
                    *examples
                        .get(example)
                        .with_context(|| format!("puzzle only has {} examples", examples.len()))?,
                ),
            )?;
        } else {
            puzzle.run_examples(&get_session()?, examples.iter().copied())?;
        };
    } else {
        puzzle.solve(&get_session()?)?;
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
