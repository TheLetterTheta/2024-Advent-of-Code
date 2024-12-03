use nom::{
    IResult,
    character::complete::{digit1, newline, space1},
    combinator::{map_res, opt},
    multi::fold_many1,
    sequence::{separated_pair, terminated},
};

use std::collections::{BinaryHeap, HashMap};

fn parse_line(input: &str) -> IResult<&str, (i32, i32)> {
    terminated(
        separated_pair(
            map_res(digit1, str::parse),
            space1,
            map_res(digit1, str::parse),
        ),
        opt(newline),
    )(input)
}

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    let (left, right) = fold_many1(
        parse_line,
        || (BinaryHeap::new(), BinaryHeap::new()),
        |mut acc, item| {
            acc.0.push(item.0);
            acc.1.push(item.1);
            acc
        },
    )(input)
    .unwrap()
    .1;

    left.into_iter_sorted()
        .zip(right.into_iter_sorted())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> i32 {
    let (left, right) = fold_many1(
        parse_line,
        || (HashMap::new(), HashMap::new()),
        |mut acc, item| {
            acc.0.entry(item.0).and_modify(|i| *i += 1).or_insert(1);
            acc.1.entry(item.1).and_modify(|i| *i += 1).or_insert(1);
            acc
        },
    )(input)
    .unwrap()
    .1;

    left.iter()
        .map(|(k, v)| k * v * right.get(&k).unwrap_or(&0))
        .sum()
}
