#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::Answer;
use std::{collections::VecDeque, io, iter::repeat_n};

pub fn main() -> Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    println!("part1: {}", part_one(&stdin)?);
    println!("part2: {}", part_two(&stdin)?);
    Ok(())
}

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    blocks: VecDeque<Block>,
    gaps: VecDeque<Block>,
}

#[derive(Debug, Clone, Copy)]
struct Block {
    start: usize,
    len: usize,
    id: i128,
}

impl Block {
    fn print(&self) {
        // print!("s{}:", self.start);
        for _i in 0..self.len {
            print!("{}", self.id);
        }
        // print!(" ");
    }

    fn checksum(&self) -> i128 {
        let mut sum = 0;
        for u in self.start..self.start + self.len {
            sum += u as i128 * self.id;
        }
        sum
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let nums: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    let mut blocks = VecDeque::new();
    let mut gaps = VecDeque::new();
    let mut gap = false;
    let mut start = 0;
    let mut id = 0;
    for n in nums {
        let block = Block {
            start,
            len: n as usize,
            id,
        };
        if n != 0 {
            if gap {
                gaps.push_back(block);
            } else {
                blocks.push_back(block);
            }
        }
        gap = !gap;
        start = start + n as usize;
        if !gap {
            id += 1;
        }
    }
    Ok(Input { blocks, gaps })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { blocks, gaps } = input;
    let mut blocks = blocks.clone();
    let mut gaps = gaps.clone();
    // 2 3g 3 3g 1 3g 3 1g 2 1g 4 1g 4 1g 3 1g 4 0g 2
    // 2    3    1    3    2    4    4    3    4    2
    //   3    3    3    1    1    1    1    1
    //
    // 2 2 1g 3 3g 1 3g 3 1g 2 1g 4 1g 4 1g 3 1g 4
    // 2 2 1 3 3g 1 3g 3 1g 2 1g 4 1g 4 1g 3 1g 3
    // 2 2 1 3 3 1 3g 3 1g 2 1g 4 1g 4 1g 3 1g
    // 2 2 1 3 3 1 3g 3 1g 2 1g 4 1g 4 1g 3
    // 2 2 1 3 3 1 3g 3 1g 2 1g 4 1g 4 1g 3

    let mut result_blocks: Vec<Block> = Vec::new();
    while let Some(gap_to_fill) = gaps.pop_front() {
        if let Some(block_to_move) = blocks.pop_back() {
            // println!("moving {:?} to gap {:?}", block_to_move, gap_to_fill);
            if block_to_move.start < gap_to_fill.start {
                blocks.push_back(block_to_move); // block was already better placed!
                break;
            }
            if gap_to_fill.len > block_to_move.len {
                result_blocks.push(Block {
                    start: gap_to_fill.start,
                    len: block_to_move.len,
                    id: block_to_move.id,
                });
                gaps.push_front(Block {
                    start: gap_to_fill.start + block_to_move.len,
                    len: gap_to_fill.len - block_to_move.len,
                    id: gap_to_fill.id,
                });
            } else if gap_to_fill.len < block_to_move.len {
                result_blocks.push(Block {
                    start: gap_to_fill.start,
                    len: gap_to_fill.len,
                    id: block_to_move.id,
                });
                blocks.push_back(Block {
                    start: block_to_move.start,
                    len: block_to_move.len - gap_to_fill.len,
                    id: block_to_move.id,
                });
            } else {
                result_blocks.push(Block {
                    start: gap_to_fill.start,
                    len: gap_to_fill.len,
                    id: block_to_move.id,
                })
            }
        }
    }
    while let Some(block) = blocks.pop_back() {
        result_blocks.push(block);
    }
    result_blocks.sort_by(|&a, &b| a.start.cmp(&b.start));
    // for b in result_blocks {
    //     b.print()
    // }
    // println!();
    let mut sum = 0;
    for b in result_blocks {
        sum += b.checksum();
    }
    Ok(Answer::Num(sum))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { blocks, gaps } = input;
    let mut blocks = blocks.clone();
    let mut gaps = gaps.clone();
    let mut result_blocks: Vec<Block> = Vec::new();
    while let Some(gap_to_fill) = gaps.pop_front() {
        if let Some(block_to_move) = blocks.pop_back() {
            // println!("moving {:?} to gap {:?}", block_to_move, gap_to_fill);
            if block_to_move.start < gap_to_fill.start {
                blocks.push_back(block_to_move); // block was already better placed!
                break;
            }
            if gap_to_fill.len > block_to_move.len {
                result_blocks.push(Block {
                    start: gap_to_fill.start,
                    len: block_to_move.len,
                    id: block_to_move.id,
                });
                gaps.push_front(Block {
                    start: gap_to_fill.start + block_to_move.len,
                    len: gap_to_fill.len - block_to_move.len,
                    id: gap_to_fill.id,
                });
            } else if gap_to_fill.len < block_to_move.len {
            } else {
                result_blocks.push(Block {
                    start: gap_to_fill.start,
                    len: gap_to_fill.len,
                    id: block_to_move.id,
                })
            }
        }
    }
    while let Some(block) = blocks.pop_back() {
        result_blocks.push(block);
    }
    result_blocks.sort_by(|&a, &b| a.start.cmp(&b.start));
    // for b in result_blocks {
    //     b.print()
    // }
    // println!();
    let mut sum = 0;
    for b in result_blocks {
        sum += b.checksum();
    }
    Ok(Answer::Num(sum))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day09_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(1928));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(6370402949053));
        Ok(())
    }
    #[bench]
    fn part_one(b: &mut Bencher) {
        part_one_impl().expect("Error");
        b.iter(|| part_one_impl())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(2858));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
