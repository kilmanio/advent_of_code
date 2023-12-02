use std::fs::read_to_string;

fn run_instruction(s: &str, accumulator: &mut isize) -> isize {
    let mut split = s.split(" ");
    let op = split.next().unwrap();
    let num = split.next().unwrap().parse::<isize>().unwrap();

    match op {
        "jmp" => return num,
        "acc" => *accumulator += num,
        _ => return 1,
    }
    return 1
}

fn change_instruction(s: &str, accumulator: &mut isize) -> isize {
    let mut split = s.split(" ");
    let op = split.next().unwrap();
    let num = split.next().unwrap().parse::<isize>().unwrap();

    match op {
        "nop" => return num,
        "acc" => *accumulator += num,
        _ => return 1,
    }
    return 1
}

fn brute(instructions: Vec<&str>, loopnum: usize) -> usize {
    let mut accumulator: isize = 0;
    let mut counter: usize = 0;
    let mut has_seen = vec![false; instructions.len()];

    while counter < instructions.len() {
        if has_seen[counter] {
            return 0;
        }
        has_seen[counter] = true;
        
        if counter == loopnum {
            counter = (counter as isize + change_instruction(instructions[counter], &mut accumulator)) as usize;
        }
        else {
            counter = (counter as isize + run_instruction(instructions[counter], &mut accumulator)) as usize;
        }
    }

    accumulator as usize
}

fn main() {
    let content = read_to_string("input").unwrap();
    let mut accumulator: usize = 0;
    let instructions = content.lines().collect::<Vec<&str>>();
    
    for i in 0..instructions.len() {
        let tmp = brute(instructions.clone(), i);
        if tmp == 0 {
            continue
        }
        accumulator = tmp;
        break
    }

    println!("accumulator = {}", accumulator);

}
