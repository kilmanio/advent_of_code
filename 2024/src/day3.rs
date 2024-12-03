use aoc_runner_derive::aoc;
use regex::Regex;

fn calculate(input: &str) -> u32 {
    let re = Regex::new(r"mul\((?<lhs>\d*),(?<rhs>\d*)\)").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            caps.name("lhs").unwrap().as_str().parse::<u32>().unwrap()
                * caps.name("rhs").unwrap().as_str().parse::<u32>().unwrap()
        })
        .sum()
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    calculate(input)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let input = re.replace_all(input, "");
    let input = input.split("don't()").next().unwrap();

    calculate(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let answer = 48;
        let hay = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(part2(hay), answer)
    }
}
