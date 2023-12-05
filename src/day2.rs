use std::{io, str::FromStr};

use log::info;

use crate::utils;

pub fn run(part: u8) -> io::Result<()> {
    let input = utils::load_input("day2.txt")?;

    let result = if part == 1 {
        part1(input)?
    } else {
        part2(input)?
    };
    info!("Result: {}", result);
    Ok(())
}

struct Game {
    id: u32,
    max_blue: u32,
    max_red: u32,
    max_green: u32,
}

#[derive(Default)]
struct Draw {
    blue: u32,
    red: u32,
    green: u32,
}

impl FromStr for Game {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game_and_values: Vec<&str> = s.split(':').collect();
        let game_id = game_and_values[0]["Game :".len() - 1..]
            .parse::<u32>()
            .unwrap();
        let sets: Vec<_> = game_and_values[1]
            .split(';')
            .map(|set| {
                let draws: Vec<_> = set
                    .split(',')
                    .map(|draw| {
                        let mut draw = draw.trim_start().split(' ');
                        let number = draw.next().unwrap().parse::<u32>().unwrap();
                        let color = draw.next().unwrap();
                        (color, number)
                    })
                    .collect();
                draws
                    .iter()
                    .fold(Draw::default(), |mut acc, (color, number)| match *color {
                        "red" => {
                            acc.red += number;
                            acc
                        }
                        "green" => {
                            acc.green += number;
                            acc
                        }
                        "blue" => {
                            acc.blue += number;
                            acc
                        }
                        _ => panic!("invalid color"),
                    })
            })
            .collect();
        Ok(Game {
            id: game_id,
            max_blue: sets.iter().max_by_key(|s| s.blue).unwrap().blue,
            max_green: sets.iter().max_by_key(|s| s.green).unwrap().green,
            max_red: sets.iter().max_by_key(|s| s.red).unwrap().red,
        })
    }
}

// 12 red cubes, 13 green cubes, and 14 blue cubes
mod maximum {
    pub const RED: u32 = 12;
    pub const GREEN: u32 = 13;
    pub const BLUE: u32 = 14;
}

fn part1(input: String) -> io::Result<u32> {
    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<io::Result<Vec<Game>>>()?;
    let result = games
        .iter()
        .filter(|g| {
            g.max_blue <= maximum::BLUE
                && g.max_green <= maximum::GREEN
                && g.max_red <= maximum::RED
        })
        .map(|g| g.id)
        .sum();
    Ok(result)
}
fn part2(input: String) -> io::Result<u32> {
    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<io::Result<Vec<Game>>>()?;
    let result = games
        .iter()
        .map(|g| g.max_blue * g.max_green * g.max_red)
        .sum();
    Ok(result)
}

#[test]
fn test_case_part1() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#
        .to_string();

    let result = part1(input).unwrap();
    assert_eq!(result, 8)
}

#[test]
fn test_case_part2() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#
        .to_string();

    let result = part2(input).unwrap();
    assert_eq!(result, 2286)
}
