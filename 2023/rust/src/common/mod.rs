#![macro_use]

use anyhow::Result;
use std::fmt::Display;
use std::{collections::HashMap, fs, path::PathBuf};

// the return type for parts sometime its Numbers sometimes its Strings
#[derive(Debug, PartialEq, Eq)]
pub enum Answer {
    Num(i128),
    #[allow(dead_code)]
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

pub struct Solver {
    solutions: HashMap<(u8, u8), fn(&str) -> Result<Answer>>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            solutions: HashMap::new(),
        }
    }
    pub fn add(&mut self, day: u8, part: u8, fun: fn(&str) -> Result<Answer>) {
        self.solutions.insert((day, part), fun);
    }
    pub fn solve(&self, opt: crate::Opt) {
        match opt {
            crate::Opt::All => self.solve_all(),
            crate::Opt::Day { day, part } => self.solve_day(day, part),
            crate::Opt::File { day, part, file } => self.solve_day_part_file(day, part, file),
        }
    }
    fn solve_all(&self) {
        for i in 1..=25 {
            self.solve_day(i, None);
        }
    }
    fn solve_day(&self, day: u8, part: Option<u8>) {
        let file = format!("src/day{:02}/input.txt", day);
        match fs::read_to_string(&file) {
            Ok(content) => {
                if let Some(part) = part {
                    self.solve_day_part_content(day, part, &content)
                } else {
                    self.solve_day_part_content(day, 1, &content);
                    self.solve_day_part_content(day, 2, &content);
                }
            }
            Err(err) => println!("Error reading {}: {}", file, err),
        }
    }
    fn solve_day_part_file(&self, day: u8, part: u8, file: PathBuf) {
        match fs::read_to_string(&file) {
            Ok(content) => self.solve_day_part_content(day, part, &content),
            Err(err) => println!("Error reading {}: {}", file.display(), err),
        }
    }
    fn solve_day_part_content(&self, day: u8, part: u8, content: &str) {
        print!("day{:02} part{:02}: ", day, part);
        if let Some(fun) = self.solutions.get(&(day, part)) {
            match fun(content) {
                Ok(answer) => println!("{}", answer),
                Err(err) => println!("Err: implementation failed with: {}", err),
            }
        } else {
            println!("Err: No solution was added to solver!");
        }
    }
}

macro_rules! regex {
    ($re:literal) => {{
        static RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            // println!("initializing regex {}", $re);
            regex::Regex::new($re).unwrap()
        });
        &RE
    }};
}

#[cfg(test)]
#[macro_use]
pub mod test_utils {
    use std::fs;
    pub fn read_from_file(filename: &str) -> String {
        println!("reading {}", filename);
        fs::read_to_string(filename)
            .unwrap_or_else(|msg| panic!("error reading {}: {}", filename, msg))
    }
    macro_rules! extract_day_from_path {
        () => {{
            let path = module_path!();
            let re = regex!(r"day\d{2}");
            let m = re
                .find(path)
                .expect("macro is only valid inside paths containing 'day\\d{2}' pattern");
            m.as_str()
        }};
    }
    macro_rules! local_file {
        ($file:literal) => {
            LazyLock::new(|| {
                test_utils::read_from_file(&format!("src/{}/{}", extract_day_from_path!(), $file))
            })
        };
    }
}
