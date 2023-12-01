mod common;
mod day00;
mod day01;

use anyhow::Result;
use common::Answer;
use std::{collections::HashMap, fs, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opt {
    All,
    Day { day: u8, part: Option<u8> },
    File { day: u8, part: u8, file: PathBuf },
}

fn main() {
    let opt = Opt::from_args();

    let mut solver = Solver::new();
    solver.add(0, 1, day00::part_one);
    solver.add(0, 2, day00::part_two);
    solver.add(1, 1, day01::part_one);
    solver.add(1, 2, day01::part_two);
    solver.solve(opt);
}

struct Solver {
    solutions: HashMap<(u8, u8), fn(&str) -> Result<Answer>>,
}

impl Solver {
    fn new() -> Solver {
        Solver {
            solutions: HashMap::new(),
        }
    }
    fn add(&mut self, day: u8, part: u8, fun: fn(&str) -> Result<Answer>) {
        self.solutions.insert((day, part), fun);
    }
    fn solve(&self, opt: Opt) {
        match opt {
            Opt::All => self.solve_all(),
            Opt::Day { day, part } => self.solve_day(day, part),
            Opt::File { day, part, file } => self.solve_day_part_file(day, part, file),
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
