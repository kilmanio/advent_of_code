use aoc_runner_derive::aoc;

fn do_line(input: &str) -> u32 {
    let mut i = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap());

    let tens = i.next().unwrap();

    tens * 10 + i.last().unwrap_or(tens)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input.split('\n').map(do_line).sum()
}

//
// I was not aware of aho-corasick

const PATTERNS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

struct NumPos {
    pub num: usize,
    pub pos: usize,
}

fn str_to_number(input: &str) -> usize {
    match input {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("NaN"),
    }
}

fn foobar(input: &str, pat: &str, first: &mut NumPos, last: &mut NumPos) {
    let tmp: Vec<usize> = input.match_indices(pat).map(|(pos, _)| pos).collect();
    if let Some(pos) = tmp.first() {
        if pos < &first.pos {
            first.pos = *pos;
            first.num = str_to_number(pat);
        }
    }
    if let Some(pos) = tmp.last() {
        if pos > &last.pos {
            last.pos = *pos;
            last.num = str_to_number(pat);
        }
    }
}

fn do_line2(input: &str) -> usize {
    let mut first = NumPos {
        num: 0,
        pos: usize::MAX,
    };
    let mut last = NumPos { num: 0, pos: 0 };

    for pattern in PATTERNS {
        foobar(input, pattern, &mut first, &mut last);
    }

    first.num * 10 + last.num
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    input.split('\n').map(do_line2).sum()
}
