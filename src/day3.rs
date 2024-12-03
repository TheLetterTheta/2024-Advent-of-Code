use itertools::Itertools;
use nom::{
    character::complete::{digit1, newline, space1},
    combinator::{map_res, opt},
    multi::fold_many1,
    sequence::{separated_pair, terminated},
    IResult,
};

use once_cell::sync::Lazy;
use regex::Regex;

use std::collections::{BinaryHeap, HashMap};

#[aoc(day3, part1)]
fn part_one(input: &str) -> u32 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
    RE.captures_iter(input)
        .map(|cap| {
            let (_, [left, right]) = cap.extract();

            let left = str::parse::<u32>(left).unwrap();
            let right = str::parse::<u32>(right).unwrap();

            left * right
        })
        .sum()
}

#[aoc(day3, part2)]
fn part_two(input: &str) -> u32 {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)|()()do\(\)|()()don't\(\)").unwrap());

    RE.captures_iter(input)
        .fold((true, 0), |(capturing, sum), cap| {
            match cap.extract::<2>() {
                ("do()", _) => (true, sum),
                ("don't()", _) => (false, sum),
                (_, [left, right]) => {
                    if capturing {
                        let left = str::parse::<u32>(left).unwrap();
                        let right = str::parse::<u32>(right).unwrap();

                        (true, sum + (left * right))
                    } else {
                        (false, sum)
                    }
                }
            }
        })
        .1
}
