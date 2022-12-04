use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| get_priority(get_deviant(line)))
        .sum()
}

fn get_deviant(line: &str) -> char {
    let len = line.len();
    let v: Vec<char> = line.chars().collect();

    for c1 in &v[..len/2] {
        for c2 in &v[len/2..] {
            if c1 == c2 { return *c1}
        }
    }
    unreachable!()
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - 96
    } else {
        (c as u32) - 38 // 65 - 27
    }
}

#[test]
fn priority_conversion() {
    let val = 'a';
    assert_eq!(get_priority(val), 1);
    let val = 'z';
    assert_eq!(get_priority(val), 26);
    let val = 'A';
    assert_eq!(get_priority(val), 27);
    let val = 'Z';
    assert_eq!(get_priority(val), 52);
}
