use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut i = 0;
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .partition(|_: &i32| {
            i += 1;
            i % 2 == 0
        });

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}
