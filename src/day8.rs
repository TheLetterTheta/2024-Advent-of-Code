use std::collections::{HashMap, HashSet};

use itertools::Itertools;

struct Input {
    bounds: (usize, usize),
    points: HashMap<char, Vec<(i16, i16)>>,
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Input {
    let _input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    Input {
        bounds: (input.lines().count(), input.lines().nth(0).unwrap().len()),
        points: input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c != '.')
                    .for_each(|(j, c)| {
                        acc.entry(c)
                            .and_modify(|vec| {
                                vec.push((i as i16, j as i16));
                            })
                            .or_insert(vec![(i as i16, j as i16)]);
                    });
                acc
            }),
    }
}

#[aoc(day8, part1)]
fn part_one(input: &Input) -> usize {
    input
        .points
        .values()
        .flat_map(|points| {
            points
                .iter()
                .tuple_combinations()
                .flat_map(|((x1, y1), (x2, y2))| {
                    vec![
                        (x1 - (x2 - x1), y1 - (y2 - y1)),
                        (x2 + (x2 - x1), y2 + (y2 - y1)),
                    ]
                })
                .filter(|&(x, y)| {
                    x >= 0 && y >= 0 && x < input.bounds.0 as i16 && y < input.bounds.1 as i16
                })
        })
        .collect::<HashSet<_>>()
        .len()
}
