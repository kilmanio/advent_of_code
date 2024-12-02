use aoc_runner_derive::aoc;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
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
        Ordering::Less => Direction::Up,
        Ordering::Equal => Direction::Unsafe,
        Ordering::Greater => Direction::Down,
    }
}

fn check_record(v: &[u32]) -> bool {
    let mut it = v.iter();
    let mut lhs = it.next().unwrap();
    let mut direction: Option<Direction> = None;

    for rhs in it {
        let this_direction = check_direction(lhs, rhs);
        if let Some(ref dir) = direction {
            if this_direction != *dir {
                return false;
            }
        }

        direction = Some(this_direction);
        lhs = rhs;
    }

    direction != Some(Direction::Unsafe)
}

fn generate_variations(sl: &[u32]) -> Vec<Vec<u32>> {
    let mut ret = Vec::new();
    let v = sl.to_vec();
    ret.push(v.clone());

    for i in 0..v.len() {
        let mut vc = v.clone();
        vc.remove(i);
        ret.push(vc);
    }

    ret
}

fn check_record_extra_life(v: &[u32]) -> bool {
    let variations = generate_variations(v);
    let mut result = false;

    for var in variations {
        if check_record(&var) {
            result = true;
        }
    }

    result
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

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .split('\n')
        .map(|s| {
            s.split(' ')
                .map(|s2| s2.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|v| check_record_extra_life(v))
        .count() as u32
}
