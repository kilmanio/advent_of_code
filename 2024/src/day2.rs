use aoc_runner_derive::aoc;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
enum Direction {
    Unsafe,
    Up,
    Down,
}

fn check_direction(lhs: &u32, rhs: &u32) -> Direction {
    let diff = lhs.abs_diff(*rhs);
    if diff > 3 {
        return Direction::Unsafe;
    }

    match lhs.cmp(rhs) {
        Ordering::Less => Direction::Down,
        Ordering::Equal => Direction::Unsafe,
        Ordering::Greater => Direction::Up,
    }
}

fn check_record(v: &[u32]) -> bool {
    let mut it = v.iter();
    let mut lhs = it.next().unwrap();
    let mut direction: Option<Direction> = None;

    for rhs in it {
        let this_direction = check_direction(lhs, rhs);
        if let Some(dir) = direction {
            if this_direction != dir {
                return false;
            }
        }

        direction = Some(this_direction);
        lhs = rhs;
    }

    true
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .split('\n')
        .map(|s| {
            s.split(' ')
                .map(|s2| s2.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|v| check_record(v))
        .count() as u32
}
