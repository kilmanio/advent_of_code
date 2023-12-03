use aoc_runner_derive::aoc;
//use rayon::prelude::*;

struct Game {
    red: usize,
    green: usize,
    blue: usize,
}

impl Game {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn to_game(input: &str) -> Game {
    let mut game = Game::new();
    let rounds = input.split(": ").nth(1).unwrap().split("; ");
    for round in rounds {
        round.split(", ").for_each(|pair| {
            let mut split = pair.split(' ');
            let count = split.next().unwrap().parse().unwrap();
            let color = split.next().unwrap();
            match color {
                "red" => {
                    game.red = std::cmp::max(game.red, count);
                }
                "green" => {
                    game.green = std::cmp::max(game.green, count);
                }
                "blue" => {
                    game.blue = std::cmp::max(game.blue, count);
                }
                _ => panic!("Not a color: {}", color),
            }
        });
    }

    game
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split('\n')
        .map(to_game)
        .enumerate()
        .filter(|(_, game)| game.is_valid())
        .map(|(id, _)| id + 1)
        .sum()
}
