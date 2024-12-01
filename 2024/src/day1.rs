use aoc_runner_derive::aoc;

pub fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut i = 0;
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .partition(|_: &u32| {
            i += 1;
            i % 2 == 1
        })
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let (left, right) = parse_input(input);

    left.iter()
        .map(|a| right.iter().filter(|b| a == *b).count() as u32 * a)
        .sum()
}
