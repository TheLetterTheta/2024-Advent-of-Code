use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

use itertools::Itertools;

type Input=Vec<Vec<i32>>;

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, complete::i32)(input)
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Input {
    separated_list1(newline, parse_line)(input).unwrap().1
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|line| {
            let mut sign = None;
            for window in line.windows(2) {
                let diff = window[0] - window[1];
                if let Some(s) = sign {
                    if diff.signum() != s {
                        return false;
                    }
                } else {
                    sign = Some(diff.signum());
                }
                if diff.abs() > 3 {
                    return false;
                }
            }
            true
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> usize {
    input
        .into_iter()
        .filter(|line|{
            for i in 0..line.len() {
                let mut new_line = (*line).clone();
                new_line.remove(i);

                if test_line(new_line) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn test_line(line: Vec<i32>) -> bool {
    let mut sign = None;
    for (l, r) in line.iter().tuple_windows() {
        let diff = l - r;
        if let Some(s) = sign {
            if diff.signum() != s {
                return false;
            }
        } else {
            sign = Some(diff.signum());
        }
        if diff.abs() > 3 {
            return false
        }
    }
    true
}
