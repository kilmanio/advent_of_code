use std::fs::read_to_string;
use itertools::Itertools;

fn part1(s: &str) -> usize {
    s
        .replace("\n","")
        .chars()
        .unique()
        .count()
}

fn part2(s: &str) -> usize {
    let mut v: Vec<char> = s
        .split("\n")
        .nth(0)
        .unwrap()
        .chars()
        .collect();

    for line in s.lines() {
        println!("line: {}, v: {:?}", line, v);
        v.retain(|&x| line.contains(x));
        println!("v after: {:?}", v);
    }

    println!("");
    v.len()
}

fn main() {
    let content = read_to_string("input").unwrap();

    let part1: usize = content
        .split("\n\n")
        .map(|g| part1(g))
        .sum();

    println!("Day1: {}", part1);

    let part2: usize = content
        .split("\n\n")
        .map(|g| part2(g))
        .sum();

    println!("Day2: {}", part2);
}
