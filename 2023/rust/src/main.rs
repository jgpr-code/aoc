#![feature(test)]
#![feature(lazy_cell)]
extern crate test;

mod common;
mod day00;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

use common::*;
use std::path::PathBuf;
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
    solver.add(2, 1, day02::part_one);
    solver.add(2, 2, day02::part_two);
    solver.add(3, 1, day03::part_one);
    solver.add(3, 2, day03::part_two);
    solver.add(4, 1, day04::part_one);
    solver.add(4, 2, day04::part_two);
    solver.add(5, 1, day05::part_one);
    solver.add(5, 2, day05::part_two);
    solver.solve(opt);
}
