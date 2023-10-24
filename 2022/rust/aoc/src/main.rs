use clap::Parser;
use std::collections::HashMap;

mod day01;
mod day02;

#[derive(Debug, Parser)]
struct Cli {
    day: Option<u8>,
    part: Option<u8>,
}

struct Solver {
    solutions: HashMap<(u8, u8), fn()>,
}

impl Solver {
    fn new() -> Solver {
        Solver {
            solutions: HashMap::new(),
        }
    }
    fn add(&mut self, day: u8, part: u8, fun: fn()) {
        self.solutions.insert((day, part), fun);
    }
    fn solve(&self, day: &Option<u8>, part: &Option<u8>) {}
    fn solve_day_part(&self, day: u8, part: u8) {}
    fn solve_day(&self, day: u8) {}
    fn solve_part(&self, part: u8) {}
    fn solve_all(&self) {}
}

fn main() {
    let cli = Cli::parse();

    let mut solver = Solver::new();
    solver.add(1, 1, day01::say_hello);
    run(&cli.day, &cli.part);
    println!("Hello, world!");
    day01::say_hello();
    //day01::say_goodbye(); not possible -> private function
    day02::say_hello();
}

fn run(day: &Option<u8>, part: &Option<u8>) {
    // no day, no part -> run all
    // no day, part -> run that part for all days
    // day, no part -> run parts for that day
    // day, part -> run that part for that day
    match (day, part) {
        (Some(day), Some(part)) => println!("day {} part {} is not implemented yet", day, part),
        (None, None) => todo!(),
        (None, Some(_)) => todo!(),
        (Some(_), None) => todo!(),
    }
}

// use cases

// cargo run r 1 1 -> 'r' for run '1' for day01 '1' for part1

// run/test day part 1
// run/test day part 2
// run/test day
// time day part 1
// time day part 2
// time day

// run all days
// time all days
