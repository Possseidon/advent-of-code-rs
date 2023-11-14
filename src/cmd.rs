use clap::Parser;

#[derive(Clone, Copy, Debug, PartialEq, Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Which year of Advent of Code to run; defaults to the current year
    #[arg(short, long)]
    pub(crate) year: Option<u32>,
    /// Which day of Advent of Code to run; defaults to the current day of December
    #[arg(short, long)]
    pub(crate) day: Option<u8>,
    /// Run part 2 of the puzzle instead of part 1
    #[arg(short('2'), long)]
    pub(crate) part2: bool,
    /// Run all or a specific example
    #[arg(short, long)]
    pub(crate) example: Option<Option<usize>>,
    /// Benchmark for N seconds; defaults to 1 second if no duration is specified
    #[arg(short, long)]
    pub(crate) bench: Option<Option<f32>>,
    /// Generate a template for the puzzle
    #[arg(short, long)]
    pub(crate) generate: bool,
}
