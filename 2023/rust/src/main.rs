#![feature(test)]
#![feature(lazy_cell)]
extern crate test;

mod common;
// mod day00;
// mod day01;
// mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
mod day18;

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
    // solver.add(0, 1, day00::part_one);
    // solver.add(0, 2, day00::part_two);
    // solver.add(1, 1, day01::part_one);
    // solver.add(1, 2, day01::part_two);
    // solver.add(2, 1, day02::part_one);
    // solver.add(2, 2, day02::part_two);
    // solver.add(3, 1, day03::part_one);
    // solver.add(3, 2, day03::part_two);
    // solver.add(4, 1, day04::part_one);
    // solver.add(4, 2, day04::part_two);
    // solver.add(5, 1, day05::part_one);
    // solver.add(5, 2, day05::part_two);
    // solver.add(6, 1, day06::part_one);
    // solver.add(6, 2, day06::part_two);
    // solver.add(7, 1, day07::part_one);
    // solver.add(7, 2, day07::part_two);
    // solver.add(8, 1, day08::part_one);
    // solver.add(8, 2, day08::part_two);
    // solver.add(9, 1, day09::part_one);
    // solver.add(9, 2, day09::part_two);
    // solver.add(10, 1, day10::part_one);
    // solver.add(10, 2, day10::part_two);
    // solver.add(11, 1, day11::part_one);
    // solver.add(11, 2, day11::part_two);
    // solver.add(12, 1, day12::part_one);
    // solver.add(12, 2, day12::part_two);
    // solver.add(13, 1, day13::part_one);
    // solver.add(13, 2, day13::part_two);
    // solver.add(14, 1, day14::part_one);
    // solver.add(14, 2, day14::part_two);
    // solver.add(15, 1, day15::part_one);
    // solver.add(15, 2, day15::part_two);
    // solver.add(16, 1, day16::part_one);
    // solver.add(16, 2, day16::part_two);
    // solver.add(17, 1, day17::part_one);
    // solver.add(17, 2, day17::part_two);
    solver.add(18, 1, day18::part_one);
    solver.add(18, 2, day18::part_two);
    solver.solve(opt);
}
