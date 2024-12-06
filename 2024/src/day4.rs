use aoc_runner_derive::aoc;

fn find_c(v: &[Vec<char>], coord: (usize, usize)) -> Option<&char> {
    v.get(coord.0)?.get(coord.1)
}

fn find_cross(v: &[Vec<char>], x: usize, y: usize) -> Option<()> {
    let t_l = (x.checked_sub(1)?, y.checked_sub(1)?);
    let t_r = (x + 1, y.checked_sub(1)?);
    let b_l = (x.checked_sub(1)?, y + 1);
    let b_r = (x + 1, y + 1);

    let t_l_c = find_c(v, t_l)?;
    let t_r_c = find_c(v, t_r)?;
    let b_l_c = find_c(v, b_l)?;
    let b_r_c = find_c(v, b_r)?;

    if (*t_l_c == 'M' && *b_r_c == 'S' || *t_l_c == 'S' && *b_r_c == 'M')
        && (*t_r_c == 'M' && *b_l_c == 'S' || *t_r_c == 'S' && *b_l_c == 'M')
    {
        return Some(());
    }

    None
}

fn find_xmas(v: &[Vec<char>], x: usize, x_dir: isize, y: usize, y_dir: isize) -> Option<()> {
    let x1 = (x as isize + x_dir) as usize;
    let x2 = (x as isize + 2 * x_dir) as usize;
    let x3 = (x as isize + 3 * x_dir) as usize;
    let y1 = (y as isize + y_dir) as usize;
    let y2 = (y as isize + 2 * y_dir) as usize;
    let y3 = (y as isize + 3 * y_dir) as usize;

    let c1 = find_c(v, (x1, y1))?;
    let c2 = find_c(v, (x2, y2))?;
    let c3 = find_c(v, (x3, y3))?;

    if *c1 == 'M' && *c2 == 'A' && *c3 == 'S' {
        return Some(());
    }

    None
}

fn find_xmas_count(v: &[Vec<char>], x: usize, y: usize) -> u32 {
    find_xmas(v, x, 1, y, 0).is_some() as u32
        + find_xmas(v, x, 1, y, 1).is_some() as u32
        + find_xmas(v, x, 0, y, 1).is_some() as u32
        + find_xmas(v, x, -1, y, 1).is_some() as u32
        + find_xmas(v, x, -1, y, 0).is_some() as u32
        + find_xmas(v, x, -1, y, -1).is_some() as u32
        + find_xmas(v, x, 0, y, -1).is_some() as u32
        + find_xmas(v, x, 1, y, -1).is_some() as u32
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

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    let mut xmas_count = 0;

    let v: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    for (x, line) in v.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            if *c == 'A' && find_cross(&v, x, y) == Some(()) {
                xmas_count += 1;
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
