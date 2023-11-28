use std::hint::unreachable_unchecked;

use crate::puzzle::{AdventOfCode, Day, Example, Part, PuzzleResult, Solution};

impl Part<1> for (AdventOfCode<2015>, Day<1>) {
    const SOLUTIONS: &'static [Solution] = &[
        Solution("count", |input| {
            let mut floor = 0;
            for char in input.bytes() {
                floor += match char {
                    b'(' => 1,
                    b')' => -1,
                    _ => panic!("invalid character"),
                }
            }
            PuzzleResult::Int(floor)
        }),
        Solution("count-unsafe", |input| {
            let mut floor = 0;
            for char in input.bytes() {
                floor += match char {
                    b'(' => 1,
                    b')' => -1,
                    _ => unsafe { unreachable_unchecked() },
                }
            }
            PuzzleResult::Int(floor)
        }),
        Solution("count-twice", |input| {
            let count = |paren| input.bytes().filter(|&char| char == paren).count() as i32;
            PuzzleResult::Int(count(b'(') - count(b')'))
        }),
        Solution("len-minus-unsafe", |input| {
            let total = input.len() as i32;
            let closing = input
                .bytes()
                .filter(|&char| match char {
                    b')' => true,
                    b'(' => false,
                    _ => unsafe { unreachable_unchecked() },
                })
                .count() as i32;
            PuzzleResult::Int(total - closing * 2)
        }),
        Solution("len-minus", |input| {
            let closing = input.bytes().filter(|&char| matches!(char, b')')).count();
            PuzzleResult::Int(input.len() as i32 - closing as i32 * 2)
        }),
        Solution("len-dec2", |input| {
            let mut count = input.len() as i32;
            for char in input.bytes() {
                if char == b')' {
                    count -= 2;
                }
            }
            PuzzleResult::Int(count)
        }),
        Solution("len-dec2-unsafe", |input| {
            let mut count = input.len() as i32;
            for char in input.bytes() {
                if char == b')' {
                    count -= 2;
                } else if char != b'(' {
                    unsafe { unreachable_unchecked() }
                }
            }
            PuzzleResult::Int(count)
        }),
        Solution("map-sum", |input| {
            PuzzleResult::Int(
                input
                    .bytes()
                    .map(|char| match char {
                        b'(' => 1,
                        b')' => -1,
                        _ => panic!("invalid character"),
                    })
                    .sum(),
            )
        }),
        Solution("map-sum-unsafe", |input| {
            PuzzleResult::Int(
                input
                    .bytes()
                    .map(|char| match char {
                        b'(' => 1,
                        b')' => -1,
                        _ => unsafe { unreachable_unchecked() },
                    })
                    .sum(),
            )
        }),
    ];

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
    const SOLUTIONS: &'static [Solution] = &[
        Solution("for-loop", |input| {
            let mut floor = 0;
            for (position, char) in input.bytes().enumerate() {
                match char {
                    b'(' => floor += 1,
                    b')' => floor -= 1,
                    _ => panic!("invalid character"),
                }
                if floor == -1 {
                    return PuzzleResult::Int(position as i32 + 1);
                }
            }
            panic!("never entered basement");
        }),
        Solution("for-loop-unsafe", |input| {
            let mut floor = 0;
            for (position, char) in input.bytes().enumerate() {
                match char {
                    b'(' => floor += 1,
                    b')' => floor -= 1,
                    _ => unsafe { unreachable_unchecked() },
                }
                if floor == -1 {
                    return PuzzleResult::Int(position as i32 + 1);
                }
            }

            unsafe { unreachable_unchecked() }
        }),
    ];

    const EXAMPLES: &'static [Example] = &[Example(21, 22), Example(23, 24)];
}
