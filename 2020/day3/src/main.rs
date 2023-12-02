use std::fs::File;
use std::io::{prelude::*, BufReader};

fn sled(right: usize, down: usize) -> usize {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let width = 31;
    let mut pos_x = 0;
    let tree_u8 = 35; //u8 of the '#' character

    let num_trees = reader
        .lines()
        .skip(down)
        .step_by(down)
        .filter(|l| {
            pos_x += right;
            if pos_x >= width {pos_x -= width};
            l.as_ref().unwrap().as_bytes()[pos_x] == tree_u8
        })
        .count();

    num_trees
}




fn main() {
    let day2;
    println!("Trees in day1: {}", sled(3,1));

    day2 = sled(1,1) * sled(3,1) * sled(5,1) * sled(7,1) * sled(1,2);

    println!("Trees in day2: {}", day2);
}

