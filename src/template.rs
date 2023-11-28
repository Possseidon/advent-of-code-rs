use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{create_dir_all, read_to_string, File, OpenOptions},
    io::{stdout, Write},
};

use anyhow::{bail, Context, Result};

use crate::puzzle::{PuzzleDay, PuzzleYear};

pub(crate) fn generate_template(year: PuzzleYear, day: PuzzleDay) -> Result<()> {
    create_template_file(year, day)?;
    add_day_to_year_mod(year, day)?;
    add_year_to_main(year)?;
    add_puzzle_to_macro(year, day)?;

    Ok(())
}

fn create_template_file(year: PuzzleYear, day: PuzzleDay) -> Result<()> {
    print!("Creating template for year {year} day {day}... ");
    stdout().flush()?;

    let year_dir = format!("src/year_{year}");
    create_dir_all(&year_dir)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("{year_dir}/day_{day}.rs"))?;

    write!(
        file,
        r#"use crate::puzzle::{{AdventOfCode, Day, Part, Solution}};

impl Part<1> for (AdventOfCode<{year}>, Day<{day}>) {{
    const SOLUTIONS: &'static [Solution] = &[Solution("solution", |input| todo!())];
}}

impl Part<2> for (AdventOfCode<{year}>, Day<{day}>) {{
    const SOLUTIONS: &'static [Solution] = &[Solution("solution", |input| todo!())];
}}
"#
    )?;

    println!("Done!");

    Ok(())
}

fn add_day_to_year_mod(year: PuzzleYear, day: PuzzleDay) -> Result<()> {
    print!("Updating mod.rs for year {year}... ");
    stdout().flush()?;

    let year_dir = format!("src/year_{year}");
    let mod_path = format!("{year_dir}/mod.rs");

    let contents = match read_to_string(&mod_path) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => Err(error)?,
    };

    let mut lines = contents.lines().collect::<BTreeSet<_>>();
    let new_day_line = format!("pub(crate) mod day_{day};");
    lines.insert(&new_day_line);

    let mut file = File::create(&mod_path)?;
    for line in lines {
        writeln!(file, "{line}")?;
    }

    println!("Done!");

    Ok(())
}

fn add_year_to_main(year: PuzzleYear) -> Result<()> {
    print!("Updating main.rs... ");
    stdout().flush()?;

    let contents = read_to_string("src/main.rs")?;

    let is_mod_line = |line: &&str| line.starts_with("mod");

    let mut mod_lines = contents
        .lines()
        .take_while(is_mod_line)
        .collect::<BTreeSet<_>>();
    let new_year_line = format!("mod year_{year};");
    mod_lines.insert(&new_year_line);

    let mut file = File::create("src/main.rs")?;
    for line in mod_lines {
        writeln!(file, "{line}")?;
    }

    for line in contents.lines().skip_while(is_mod_line) {
        writeln!(file, "{line}")?;
    }

    println!("Done!");

    Ok(())
}

fn add_puzzle_to_macro(year: PuzzleYear, day: PuzzleDay) -> Result<()> {
    print!("Updating puzzle.rs... ");
    stdout().flush()?;

    let contents = read_to_string("src/puzzle.rs")?;

    let is_puzzle_macro_start = |line: &&str| line.starts_with("puzzles! {");

    let puzzle_lines = contents
        .lines()
        .skip_while(|line| !is_puzzle_macro_start(line))
        .skip(1)
        .take_while(|line| !line.starts_with('}'))
        .collect::<BTreeSet<_>>();

    let mut puzzles = puzzle_lines
        .into_iter()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            let year = iter.next().context("year not found")?.parse()?;
            if iter.next() != Some("=>") {
                bail!("`=>` expected");
            }
            if iter.next() != Some("[") {
                bail!("`[` expected");
            }
            if iter.next_back() != Some("]") {
                bail!("`]` expected");
            }
            Ok((year, iter.map(|day| day.parse()).collect::<Result<_, _>>()?))
        })
        .collect::<Result<BTreeMap<PuzzleYear, BTreeSet<PuzzleDay>>, _>>()?;

    puzzles.entry(year).or_default().insert(day);

    let mut file = File::create("src/puzzle.rs")?;
    let content = contents
        .lines()
        .take_while(|line| !is_puzzle_macro_start(line));
    for line in content {
        writeln!(file, "{line}")?;
    }

    writeln!(file, "puzzles! {{")?;
    for (year, days) in puzzles {
        write!(file, "    {year} => [ ")?;
        for day in days {
            write!(file, "{day} ")?;
        }
        writeln!(file, "]")?;
    }
    writeln!(file, "}}")?;

    println!("Done!");

    Ok(())
}
