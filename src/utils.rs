use std::{fs, io};

pub fn load_input(filename: &str) -> io::Result<String> {
    fs::read_to_string(format!("inputs/{filename}"))
}
