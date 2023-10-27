use std::{fmt::Display, fs};

// the return type for parts sometime its Numbers sometimes its Strings
#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Answer {
    Num(i128),
    Str(String),
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Answer::Num(i) => write!(f, "{}", i),
            Answer::Str(s) => write!(f, "{}", s),
        }
    }
}

// small utility to use for reading the input and test files in tests
pub fn read_from_file(filename: &str) -> String {
    println!("reading {}", filename);
    fs::read_to_string(filename).unwrap_or_else(|msg| panic!("error reading {}: {}", filename, msg))
}
