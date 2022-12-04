use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let (elf_number, elf_calories) = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    elf_calories
}
