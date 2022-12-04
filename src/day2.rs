use aoc_runner_derive::aoc;

fn play(input: &str) -> (u32 ,u32) {
    match input {
        "A X" => (4, 4),
        "A Y" => (1, 8),
        "A Z" => (7, 3),
        "B X" => (8, 1),
        "B Y" => (5, 5),
        "B Z" => (2, 9),
        "C X" => (3, 7),
        "C Y" => (9, 2),
        "C Z" => (6, 6),
        _ => unreachable!(),
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| play(line).1)
        .sum()
}

fn play2(input: &str) -> (u32 ,u32) {
    match input {
        "A X" => (7, 3),
        "A Y" => (4, 4),
        "A Z" => (1, 8),
        "B X" => (8, 1),
        "B Y" => (5, 5),
        "B Z" => (2, 9),
        "C X" => (9, 2),
        "C Y" => (6, 6),
        "C Z" => (1, 7),
        _ => unreachable!(),
    }
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| play2(line).1)
        .sum()
}
