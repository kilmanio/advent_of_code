use std::fs::File;
use std::io::{prelude::*, BufReader};

fn populate() -> Vec<i32> {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn resolve2(numbers: Vec<i32>) {
    let goal = 2020;
    let mut result = 0;

    for number in &numbers {
        let target = goal - number;
        if numbers.contains(&target) {
            result = number * target;
            break;
        }
    }
    println!("2 number sum to 2020 multiplied: {}", result);
}

fn resolve3(numbers: Vec<i32>) {
    let goal = 2020;
    let mut result = 0;

    for number in &numbers {
        let target = goal - number;
        for number2 in &numbers {
            let target2 = target - number2;
            if numbers.contains(&target2) {
                result = number * number2 * target2;
                break;
            }
        }
    }
    println!("3 number sum to 2020 multiplied: {}", result);
}

fn main() {
    let numbers: Vec<i32>;
    numbers = populate();
    resolve2(numbers.clone());
    resolve3(numbers.clone());
}

