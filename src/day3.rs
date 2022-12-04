use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| get_priority(get_deviant(line)))
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let mut iter = input.lines();

    let mut sum = 0;

    //Presumably, the #input is a multiple of 3, so this is safe :^)
    'outer: while let Some(l1) = iter.next() {
        let l2 = iter.next().unwrap();
        let l3 = iter.next().unwrap();

        for c1 in l1.chars() {
            for c2 in l2.chars() {
                if c1 == c2 {
                    for c3 in l3.chars() {
                        if c1 == c3 {
                            sum += get_priority(c1);
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }
    sum
}

fn get_deviant(line: &str) -> char {
    let len = line.len();
    let v: Vec<char> = line.chars().collect();

    for c1 in &v[..len / 2] {
        for c2 in &v[len / 2..] {
            if c1 == c2 {
                return *c1;
            }
        }
    }
    unreachable!()
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - (b'a' as u32) + 1
    } else {
        (c as u32) - (b'A' as u32) + 27
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
