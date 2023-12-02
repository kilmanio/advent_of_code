use std::fs::read_to_string;

struct BoardingInfo {
    row: u16,
    column: u16,
    id: u16,
}

fn mediate(lower: &mut u16, higher: &mut u16, up: &bool) {
    let delta = (*lower + *higher + 1) / 2 - *lower;
    if *up {
        *lower += delta;
    }
    else {
        *higher -= delta;
    }
}

fn format(c: char) -> bool {
    if c == 'F' || c == 'L' {
        return false;
    }
    true
}

fn calc_info(s: &str) -> BoardingInfo {
    let mut rl: u16 = 0;
    let mut rh: u16 = 127;
    s[..7]
        .chars()
        .for_each(|c| mediate(&mut rl, &mut rh, &format(c)));

    let mut cl: u16 = 0;
    let mut ch: u16 = 7;
    s[7..]
        .chars()
        .for_each(|c| mediate(&mut cl, &mut ch, &format(c)));

    let info = BoardingInfo {
        row: rl,
        column: cl,
        id: rl*8+cl,
    };

    info
}

fn gauss_sum(num: u32) -> u32 {
    ((1-(num%2))+num)*(num/2)+num*(num%2)
}

fn main() {
    let content = read_to_string("input").unwrap();

    let info: Vec<_> = content
        .lines()
        .map(|s| calc_info(s))
        .collect();

    let high: u32 = info
        .iter()
        .map(|i| i.id)
        .max()
        .unwrap() as u32;

    let low: u32 = info
        .iter()
        .map(|i| i.id)
        .min()
        .unwrap() as u32;

    let sum: u32 = info
        .iter()
        .map(|i| i.id as u32)
        .sum::<u32>();

    let expected = gauss_sum(high) - gauss_sum(low-1);

    dbg!(gauss_sum(8));

    println!("The highest ID = {}", high);
    println!("The lowest ID = {}", low);
    println!("there are {} IDs", high-low);
    println!("the total sum should be {}", expected);
    println!("the total sum IS {}, so the defecit = {}", sum, expected-sum);
}
