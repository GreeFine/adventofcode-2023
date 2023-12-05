use std::{io, str::Chars};

use log::{debug, info};

use crate::utils;

pub fn run(part: u8) -> io::Result<()> {
    if part == 1 {
        part1()
    } else {
        let input = utils::load_input("day1.txt")?;
        let result = part2(input)?;
        info!("Result: {}", result);
        Ok(())
    }
}

fn part1() -> io::Result<()> {
    let input = utils::load_input("day1.txt")?;

    let result: i64 = input
        .lines()
        .map(|line| {
            let numbers: String = line.chars().filter(|c| c.is_numeric()).collect();
            // using number_1 * 10 + number_2, to replicate a concat instead of an addition
            debug!(
                "{line} - n1: {}, n2: {}",
                numbers[..1].parse::<i64>().unwrap(),
                numbers[numbers.len() - 1..].parse::<i64>().unwrap()
            );
            Ok((numbers[..1].parse::<i64>().unwrap() * 10)
                + numbers[numbers.len() - 1..].parse::<i64>().unwrap())
        })
        .collect::<io::Result<Vec<i64>>>()?
        .into_iter()
        .sum();
    info!("Result: {}", result);
    Ok(())
}

/// Check if there is an ascii number or a part of it and return it numeric version
/// Return and error is no parts where found. Will return none if the it's a part but not complete number
fn number_in_str(value: &str) -> Result<Option<u32>, ()> {
    const NUMBERS_ASCI: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let Some(&matching) = NUMBERS_ASCI.iter().find(|n| n.starts_with(value)) else {
        return Err(());
    };
    Ok((value.len() == matching.len()).then(|| match matching {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("should not be possible to be here without a match on a number"),
    }))
}

struct AsciOrNumeric<'a> {
    input: Chars<'a>,
    buffer: String,
}

impl<'a> AsciOrNumeric<'a> {
    fn new(input: Chars<'a>) -> Self {
        Self {
            input,
            // Longest possible valid ascii number
            buffer: String::with_capacity("three".len()),
        }
    }
}

impl Iterator for AsciOrNumeric<'_> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        'char: loop {
            let c = self.input.next()?;

            let digit = c.to_digit(10);
            if digit.is_some() {
                self.buffer.clear();
                return digit;
            }

            self.buffer.push(c);
            let ascii_digit = number_in_str(&self.buffer);
            // Found only a part of a number
            if Ok(None) == ascii_digit {
                continue 'char;
            };

            // Prune chars that aren't parts of number
            'prune: loop {
                self.buffer.remove(0);
                let ascii_digit = number_in_str(&self.buffer);
                // Found only a part of a number
                if self.buffer.is_empty() || Ok(None) == ascii_digit {
                    break 'prune;
                };
            }

            // Found a number
            if let Ok(Some(ascii_digit)) = ascii_digit {
                return Some(ascii_digit);
            };
        }
    }
}

fn part2(input: String) -> io::Result<u32> {
    let result: u32 = input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = AsciOrNumeric::new(line.chars()).collect();
            // using number_1 * 10 + number_2, to replicate a concat instead of an addition
            debug!(
                "{line} - n1: {}, n2: {}\nnumbers: {:?}",
                numbers.first().unwrap(),
                numbers.last().unwrap(),
                numbers
            );
            Ok((numbers.first().unwrap() * 10) + numbers.last().unwrap())
        })
        .collect::<io::Result<Vec<u32>>>()?
        .into_iter()
        .sum();
    Ok(result)
}

#[test]
fn test_part_two() {
    std::env::set_var("RUST_LOG", "DEBUG");
    pretty_env_logger::init();
    let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    let result = part2(input.to_string()).unwrap();
    assert_eq!(result, 281)
}

#[test]
fn test_part_two_edge_case() {
    std::env::set_var("RUST_LOG", "DEBUG");
    pretty_env_logger::init();

    let input = "xfoneightsixnine6fiveseven";
    let result = part2(input.to_string()).unwrap();
    assert_eq!(result, 17)
}
