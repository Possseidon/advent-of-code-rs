use anyhow::{bail, Result};

use crate::puzzle::{AdventOfCode, Day, Example, Part};

impl Part<1> for (AdventOfCode<2015>, Day<1>) {
    fn solve(input: &str) -> Result<String> {
        Ok(input
            .bytes()
            .map(|char| match char {
                b'(' => Ok(1),
                b')' => Ok(-1),
                _ => bail!("invalid character"),
            })
            .sum::<Result<i32, _>>()?
            .to_string())
    }

    const EXAMPLES: &'static [Example] = &[
        Example(3, 5),
        Example(4, 5),
        Example(6, 8),
        Example(7, 8),
        Example(9, 10),
        Example(11, 13),
        Example(12, 13),
        Example(14, 16),
        Example(15, 16),
    ];
}

impl Part<2> for (AdventOfCode<2015>, Day<1>) {
    fn solve(input: &str) -> Result<String> {
        let mut floor = 0;
        for (position, char) in input.bytes().enumerate() {
            match char {
                b'(' => floor += 1,
                b')' => floor -= 1,
                _ => bail!("invalid character"),
            }
            if floor == -1 {
                return Ok((position + 1).to_string());
            }
        }
        bail!("never entered basement");
    }

    const EXAMPLES: &'static [Example] = &[Example(21, 22), Example(23, 24)];
}
