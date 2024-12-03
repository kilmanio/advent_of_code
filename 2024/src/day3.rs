use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((?<lhs>\d*),(?<rhs>\d*)\)").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            caps.name("lhs").unwrap().as_str().parse::<u32>().unwrap()
                * caps.name("rhs").unwrap().as_str().parse::<u32>().unwrap()
        })
        .sum()
}
