use std::fs::read_to_string;

fn check_bounds(val: &str, lower: i32, higher: i32) -> bool {
    let intval = val.parse::<i32>().unwrap();
    let check = intval >= lower && intval <= higher;
    //println!("{} between {} and {} : {}", intval, lower, higher, check);
    return check
}

fn check_hgt(val: &str) -> bool {
    let len = val.len();
    if len < 4 {
        return false
    }

    let num = &val[0..len-2];

    if val.chars().nth(len-1).unwrap() == 'm' {
        let check = check_bounds(num, 150, 193);
        //println!("hgt = {} cm, and {}", num, check);
        return check
    }

    if val.chars().nth(len-1).unwrap() == 'n' {
        let check = check_bounds(num, 59, 76);
        //println!("hgt = {} in, and {}", num, check);
        return check
    }

    false
}

fn check_hcl(val: &str) -> bool {
    if val.len() != 7 {
        return false
    }

    if val.chars().nth(0).unwrap() != '#' {
        return false
    }
   
    let check = val.chars()
        .skip(1)
        .filter(|c| match c {
            '0' | '1' | '2' | '3' | '4' | 
                '5' | '6' | '7' | '8' | '9' | 
                'a' | 'b' | 'c' | 'd' | 'e' | 
                'f' => true,
            _ => false,
        })
        .count() == 6;
    //if check {println!("hcl pass")};
    check
}

fn check_ecl(val: &str) -> bool {
    let check = match val {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => 
            true, 
        _ => false,
    };
    //if check {println!("ecl pass")};
    check
}

fn check_pid(val: &str) -> bool {
    if val.len() != 9 {
        return false
    }
    let check = check_bounds(val, 0, 999999999);
    //if check {println!("pid pass")};
    check
}

fn validate(raw: String) -> bool {
    
    let info: Vec<_> = raw
        .split_whitespace()
        .collect();

    if info.len() < 7 {
        return false
    }

    /*
    if info.len() == 8 {
        return true
    }
    */
    /*
    let no_cid = info
        .iter()
        .filter(|f| {
             &f[..3] == "cid"
         })
        .count() == 0;
    */

    let valid = info
        .iter()
        .filter(|f| {
            match &f[..3] {
                "byr" => check_bounds(&f[4..], 1920, 2002),
                "iyr" => check_bounds(&f[4..], 2010, 2020),
                "eyr" => check_bounds(&f[4..], 2020, 2030),
                "hgt" => check_hgt(&f[4..]),
                "hcl" => check_hcl(&f[4..]),
                "ecl" => check_ecl(&f[4..]),
                "pid" => check_pid(&f[4..]),
                _ => false,
            }
        })
            
        .count();

    //dbg!(valid);
    return valid == 7
}

fn format_string(info: String) -> String {
    info.replace("\n", " ")
}

fn main() {
    let content = read_to_string("input").unwrap();

    let valid_passports = content
        .split("\n\n")
        .filter(|l| validate(format_string(l.to_string())))
        .count();

    println!("Amount of valid passports in p2: {}", valid_passports)
}
