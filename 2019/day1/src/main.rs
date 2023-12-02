use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut temp: u32;
    let mut total: u32 = 0;

    for line in reader.lines() {
        temp = line.unwrap().parse::<u32>().unwrap();
        while temp >= 9 {
            temp = temp/3;
            temp -= 2;
            total += temp;
        }
    }

    println!("{}", total);

    Ok(())
}

