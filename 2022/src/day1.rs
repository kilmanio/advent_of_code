use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let elf_calories = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max_by(|a, b| a.cmp(b))
        .unwrap();
    elf_calories
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();

    calories.sort();

    calories.pop().unwrap() + calories.pop().unwrap() + calories.pop().unwrap()
}
