use std::collections::HashMap;

use super::common::*;
use anyhow::Result;

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    board: Vec<Vec<char>>,
}

fn parse_input(input: &str) -> Result<Input> {
    let board = input.lines().map(|l| l.chars().collect()).collect();

    Ok(Input { board })
}

fn compute_board_load(board: &Vec<Vec<char>>) -> usize {
    let rows = board.len();
    let cols = board[0].len();
    let mut load = 0;
    for r in 0..rows {
        for c in 0..cols {
            if board[r][c] == 'O' {
                load += rows - r;
            }
        }
    }
    load
}

fn print_board(board: &Vec<Vec<char>>) {
    for r in 0..board.len() {
        for c in 0..board[0].len() {
            print!("{}", board[r][c]);
        }
        println!();
    }
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { board } = input;
    // start with last row and move up
    // index and current amount of moving rocks
    let rows = board.len();
    let cols = board[0].len();
    let mut new_board = board.clone(); // should be deep copy
    for r in 0..new_board.len() {
        for c in 0..new_board[0].len() {
            if new_board[r][c] == 'O' {
                new_board[r][c] = '.';
            }
        }
    }
    let mut moving_rocks = vec![0; cols];
    for r in (0..rows).rev() {
        // println!("{} {:?}", r, moving_rocks);
        for c in 0..cols {
            match board[r][c] {
                '#' => {
                    let mut to_place = moving_rocks[c];
                    let mut rbelow = r + 1;
                    while to_place > 0 && rbelow < rows {
                        new_board[rbelow][c] = 'O';
                        to_place -= 1;
                        rbelow += 1;
                    }
                    moving_rocks[c] = 0;
                }
                'O' => moving_rocks[c] += 1,
                _ => {
                    // println!("ignoring {}", board[r][c])
                }
            }
        }
    }
    for c in 0..cols {
        let mut to_place = moving_rocks[c];
        let mut rbelow = 0;
        while to_place > 0 && rbelow < rows {
            new_board[rbelow][c] = 'O';
            to_place -= 1;
            rbelow += 1;
        }
        moving_rocks[c] = 0;
    }
    print_board(board);
    println!();
    print_board(&new_board);
    Ok(Answer::Num(compute_board_load(&new_board) as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    todo!();
    Ok(Answer::Num(-1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test.txt");
    static INPUT: LazyLock<String> = local_file!("input.txt");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(136));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        b.iter(|| part_one())
    }
    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        b.iter(|| part_two())
    }
}
