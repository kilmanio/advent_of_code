use aoc_runner_derive::aoc;

fn do_line(input: &str) -> u32 {
    let mut i = input.chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap());

    let tens = i.next().unwrap();

    tens*10+i.last().unwrap_or(tens)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input.split("\n")
        .map(|l| do_line(l))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    todo!()
}
