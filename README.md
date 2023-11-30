# Advent of Code

Using [Rust](https://www.rust-lang.org/) 🦀 to solve [Advent of Code](https://adventofcode.com/) 🎄

The project is a single CLI tool that comes with a bunch of useful features:

- ⭐ [Usable](#usage) for [Any Year](https://adventofcode.com/events)
- 🌐 Input Download
- ✅ Example Validation
- ⚡ [Solution Benchmarking](#benchmarking)
- 🔍 [Benchmark Comparison](#benchmark-comparison)
- 📄 [Template Generation](#template-generation)

Feel free to fork and use it for your own solutions. :)

## Setup

Log into [Advent of Code](https://adventofcode.com/) and use dev-tools to grab your session token from the browser cookies and put it in a `.env` file:

```sh
ADVENT_OF_CODE_SESSION=8eb8a089a9a42d37233365fda7716a86c5274176b841950694cf87a99cee5aeee63e1dd4fdc8d9e8bf6fced939220674dfe651a4eaa8949a93e21f8347767957
```

The token should last for a full year, so you'll only need to refresh it for the next year's Advent of Code.

Note, that puzzle input is **always** downloaded live and never cached/stored.

## Usage

Simply use `cargo` to run a puzzle's solution. It defaults to running the solution of the current day of December.

```sh
cargo run
```

More options can be added after a `--`:

```sh
cargo run -- --year 2015 --day 1
```

A full list of all options, which can also be viewed using `-h`:

```txt
-y, --year <YEAR>          Which year of Advent of Code to run; defaults to the current year
-d, --day <DAY>            Which day of Advent of Code to run; defaults to the current day of December
-2, --part2                Run part 2 of the puzzle instead of part 1
-s, --solution <SOLUTION>  Which solution to run; defaults to the first solution
-e, --example [<EXAMPLE>]  Run all or a specific example
-b, --bench [<BENCH>]      Benchmark for N seconds; defaults to 1 second if no duration is specified
-c, --compare              Compare benchmark results for alternatives
-g, --generate             Generate a template for the puzzle
-h, --help                 Print help
-V, --version              Print version
```

## Benchmarking

Benchmarking is done with the `--bench` flag followed by an optional number of seconds to run (defaults to 1 second). I recommend you to build in `--release` mode (note the `-r` flag in the example below). Running benchmarks in debug mode will print a yellow warning, in case you forget.

```sh
cargo run -r -- --year 2015 -d 1 --bench 1.0
```

This will produce benchmark results that look something like this:

```txt
Advent of Code 2015 - Day 1 - Part 1

Grabbing input... got 7000 bytes.

Benchmark ran for 982.21ms (plus 17.81ms of overhead)
  Iterations: 45,461
  Avg±StdDev: 21.61µs ± 26.00ns
 Min<Med<Max: 18.60µs < 20.00µs < 406.60µs
```

### Benchmark Comparison

If a puzzle has multiple solutions, they can be compared with the `--compare` flag:

```sh
cargo run -r -- --year 2015 -d 1 --bench --compare
```

This will run all solutions one after the other and print a list of results sorted by their average runtime:

```txt
Advent of Code 2015 - Day 1 - Part 1

Grabbing input... got 7000 bytes.

                  ┏━━ Averge ±   StdDev ┯ Relative ┳━ Mininum ┯━━ Median ┯━ Maximum ┓
┏━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━┿━━━━━━━━━━╋━━━━━━━━━━┿━━━━━━━━━━┿━━━━━━━━━━┫
┃ len-minus       ┃   1.05µs ±   1.00ns │     0.0% ┃ 900.00ns │   1.00µs │ 301.70µs ┃
┃ len-dec2        ┃   1.37µs ±   1.00ns │    30.9% ┃   1.20µs │   1.30µs │ 247.00µs ┃
┃ count-unsafe    ┃   1.38µs ±   2.00ns │    31.0% ┃   1.20µs │   1.30µs │ 320.20µs ┃
┃ map-sum-unsafe  ┃   1.43µs ±   3.00ns │    35.7% ┃   1.20µs │   1.30µs │ 898.70µs ┃
┃ len-dec2-unsafe ┃   1.43µs ±   2.00ns │    36.4% ┃   1.20µs │   1.30µs │ 274.00µs ┃
┃ count-twice     ┃   2.11µs ±   2.00ns │   100.5% ┃   1.80µs │   1.90µs │ 273.00µs ┃
┃ map-sum         ┃  16.65µs ±  19.00ns │  1485.6% ┃  14.20µs │  15.20µs │ 248.70µs ┃
┃ count           ┃  21.61µs ±  28.00ns │  1958.3% ┃  18.60µs │  20.00µs │ 302.40µs ┃
┗━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━━━┷━━━━━━━━━━┻━━━━━━━━━━┷━━━━━━━━━━┷━━━━━━━━━━┛
```

## Template Generation

If a puzzle does not have a solution yet, a template can be generated for it with the `--generate` flag:

```sh
cargo run -- --generate --year 2015 -d 1
```

As per usual, the `year` and `day` default to the current year and day of December.

⚠ Make sure to run this from the project root, as it edits some source files!
