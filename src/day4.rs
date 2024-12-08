use itertools::Itertools;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

#[aoc(day4, part1)]
fn part_one(input: &Vec<String>) -> usize {
    let mut search_strs: usize = input
        .iter()
        .map(|s| s.matches("XMAS").count() + s.matches("SAMX").count())
        .sum();

    // Vertical
    for i in 0..input.len() {
        let mut s = String::new();
        for j in 0..input[i].len() {
            s.push(input[j].chars().nth(i).unwrap());
        }
        search_strs += s.matches("XMAS").count() + s.matches("SAMX").count();
    }

    // Diagonal
    for i in 0..input.len() {
        let mut s = String::new();
        let mut s2 = String::new();
        let mut r = String::new();
        let mut r2 = String::new();
        for j in 0..=i {
            s.push(input[i - j].chars().nth(j).unwrap());
            if i != input.len() - 1 {
                s2.push(input[input.len() - 1 - (i - j)].chars().nth(j).unwrap());
                if j != input.len() - 1 {
                    r2.push(
                        input[input.len() - 1 - (i - j)]
                            .chars()
                            .nth(input.len() - 1 - j)
                            .unwrap(),
                    );
                }
            }
            if j != input.len() {
                r.push(input[i - j].chars().nth(input.len() - 1 - j).unwrap());
            }
        }
        search_strs += s.matches("XMAS").count() + s.matches("SAMX").count();
        search_strs += r.matches("XMAS").count() + r.matches("SAMX").count();
        search_strs += r2.matches("XMAS").count() + r2.matches("SAMX").count();
        search_strs += s2.matches("XMAS").count() + s2.matches("SAMX").count();
    }
    search_strs
}

#[aoc(day4, part2)]
fn part_two(input: &Vec<String>) -> usize {
    input
        .iter()
        .enumerate()
        .take(input.len() - 1)
        .skip(1)
        .map(|(i, line)| 
            line.chars()
                .enumerate()
                .take(line.len() - 1)
                .skip(1)
                .filter(|(j, c)| {
                    if *c != 'A' {
                        return false;
                    }

                    let tl = input[i - 1]
                        .chars()
                        .nth(j - 1)
                        .expect("Can't not exist lol");
                    let tr = input[i - 1]
                        .chars()
                        .nth(j + 1)
                        .expect("Can't not exist lol");
                    let br = input[i + 1]
                        .chars()
                        .nth(j + 1)
                        .expect("Can't not exist lol");
                    let bl = input[i + 1]
                        .chars()
                        .nth(j - 1)
                        .expect("Can't not exist lol");

                    matches!(tl, 'M' | 'S')
                        && matches!(tr, 'M' | 'S')
                        && matches!(br, 'M' | 'S')
                        && matches!(bl, 'M' | 'S')
                        && tl != br
                        && tr != bl
                })
                .count()
        )
        .sum::<usize>()
}
