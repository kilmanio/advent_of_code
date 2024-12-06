use aoc_runner_derive::aoc;

//hurray for quadratic complexity :)))
fn check_update(update: &[u32], rules: &[(u32, u32)]) -> bool {
    for (idx, cur) in update.iter().enumerate() {
        for prev in update[..idx].iter() {
            if rules.contains(&(*cur, *prev)) {
                return false;
            }
        }
    }
    true
}

fn middle_number(slice: &[u32]) -> u32 {
    slice[slice.len() / 2]
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    let mut split = input.split("\n\n");
    let rules: Vec<(u32, u32)> = split
        .next()
        .unwrap()
        .split('\n')
        .map(|a| {
            let mut split2 = a.split("|");
            (
                split2.next().unwrap().parse().unwrap(),
                split2.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    split
        .next()
        .unwrap()
        .split('\n')
        .map(|update_string| {
            update_string
                .split(',')
                .map(|ascii| ascii.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|update| check_update(update, &rules))
        .map(|update| middle_number(&update))
        .sum()
}
