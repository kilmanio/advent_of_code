use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Passline {
    lower: usize,
    upper: usize,
    character: u8,
    password: String,
}

fn read_line(line: String) -> Option<Passline> {
    let mut split = line.split(' ');
    let mut numbers = split.next()?.split('-');
    let ret = Passline { 
        lower: numbers.next()?.parse::<usize>().ok()?,
        upper: numbers.next()?.parse::<usize>().ok()?,
        character: split.next()?.as_bytes()[0], 
        password: split.next()?.to_string()
    };

    Some(ret)
}

fn is_valid(info: &Passline) -> bool {
    let count = info.password
        .as_bytes()
        .iter()
        .filter(|&&c| c == info.character)
        .count() as usize;

    if count < info.lower || count > info.upper {
        return false;
    }

    true
}

fn is_valid2(info: &Passline) -> bool {
    let check1 = info.password.as_bytes()[info.lower-1] == info.character;
    let check2 = info.password.as_bytes()[info.upper-1] == info.character;

    check1 ^ check2
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let num = reader
        .lines()
        .filter_map(|x| read_line(x.unwrap()))
        .filter(|x| is_valid(x))
        .count();

    println!("Amount of valid passwords part one: {}", num);

    let file2 = File::open("input").unwrap();
    let reader2 = BufReader::new(file2);

    let num2 = reader2
        .lines()
        .filter_map(|x| read_line(x.unwrap()))
        .filter(|x| is_valid2(x))
        .count();

    println!("Amount of valid passwords part two: {}", num2);
}

