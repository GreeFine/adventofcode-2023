use std::{env, io};

mod day1;
mod day2;
mod utils;

use clap::Parser;

/// Advent of code 2023 Solutions
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // Day to be launched
    day: u8,
    #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
    #[arg(default_value_t = 1)]
    /// Select which part of the day will be run
    part: u8,
}

fn main() -> io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let cli = Cli::parse();

    match cli.day {
        1 => {
            day1::run(cli.part)?;
        }
        2 => {
            day2::run(cli.part)?;
        }
        _ => unimplemented!(),
    };
    Ok(())
}
