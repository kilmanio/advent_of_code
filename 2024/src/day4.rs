use aoc_runner_derive::aoc;

fn find_xmas(v: &Vec<Vec<char>>, x: usize, x_dir: isize, y: usize, y_dir: isize) -> u32 {
    let x1 = (x as isize + x_dir) as usize;
    let x2 = (x as isize + 2 * x_dir) as usize;
    let x3 = (x as isize + 3 * x_dir) as usize;
    let y1 = (y as isize + y_dir) as usize;
    let y2 = (y as isize + 2 * y_dir) as usize;
    let y3 = (y as isize + 3 * y_dir) as usize;

    let mut m = false;
    let mut a = false;
    let mut s = false;

    if let Some(line) = v.get(x1) {
        if let Some(c) = line.get(y1) {
            if *c == 'M' {
                m = true;
            }
        }
    }
    if let Some(line) = v.get(x2) {
        if let Some(c) = line.get(y2) {
            if *c == 'A' {
                a = true;
            }
        }
    }
    if let Some(line) = v.get(x3) {
        if let Some(c) = line.get(y3) {
            if *c == 'S' {
                s = true;
            }
        }
    }

    (m & a & s) as u32
} 

fn find_xmas_count(v: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    find_xmas(v, x, 1, y, 0)
        + find_xmas(v, x, 1, y, 1)
        + find_xmas(v, x, 0, y, 1)
        + find_xmas(v, x, -1, y, 1)
        + find_xmas(v, x, -1, y, 0)
        + find_xmas(v, x, -1, y, -1)
        + find_xmas(v, x, 0, y, -1)
        + find_xmas(v, x, 1, y, -1)
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    let mut xmas_count = 0;

    let v: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    for (x, line) in v.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            if *c == 'X' {
                xmas_count += find_xmas_count(&v, x, y);
            }
        }
    }

    xmas_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let answer = 18;
        let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(part1(input), answer);
    }
}
