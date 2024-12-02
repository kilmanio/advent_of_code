use aoc_runner_derive::aoc;
use std::cmp::Ordering;

fn check_safe(v: &[u32]) -> bool {
    let mut it = v.iter();
    let mut lhs = it.next().unwrap();
    let mut ordering = Ordering::Equal;

    for rhs in it {
        let this_ordering = lhs.cmp(rhs);

        if (this_ordering as i8).abs_diff(ordering as i8) > 1 {
            return false;
        }
        ordering = this_ordering;

        let diff = lhs.abs_diff(*rhs);
        if diff < 1 || diff > 3 {
            return false;
        }

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
        .filter(|v| check_safe(v))
        .count() as u32
}
