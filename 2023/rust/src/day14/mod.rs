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

// fn print_board(board: &Vec<Vec<char>>) {
//     for r in 0..board.len() {
//         for c in 0..board[0].len() {
//             print!("{}", board[r][c]);
//         }
//         println!();
//     }
// }

fn hash_board(board: &Vec<Vec<char>>) -> String {
    let mut hash = String::new();
    for r in 0..board.len() {
        let line = String::from_iter(board[r].iter());
        hash = format!("{}{}", hash, line);
    }
    hash
}

fn unhash_board(hash: &String, rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut board = vec![vec!['.'; cols]; rows];
    let mut si = 0;
    let cs: Vec<char> = hash.chars().collect();
    for r in 0..rows {
        for c in 0..cols {
            board[r][c] = cs[si];
            si += 1;
        }
    }
    board
}

fn empty_rocks(board: &mut Vec<Vec<char>>) {
    for r in 0..board.len() {
        for c in 0..board[0].len() {
            if board[r][c] == 'O' {
                board[r][c] = '.';
            }
        }
    }
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { board } = input;
    let mut a = HashMap::new();
    let new_board = tilt_north(board, &mut a);
    // print_board(board);
    // println!();
    // print_board(&new_board);
    Ok(Answer::Num(compute_board_load(&new_board) as i128))
}

fn tilt_north(board: &Vec<Vec<char>>, memo: &mut HashMap<String, String>) -> Vec<Vec<char>> {
    // bottom to top
    let rows = board.len();
    let cols = board[0].len();
    let hash = hash_board(board);
    if memo.contains_key(&hash) {
        let hashed_board = memo.get(&hash).unwrap();
        return unhash_board(hashed_board, rows, cols);
    }
    let mut new_board = board.clone(); // should be deep copy
    empty_rocks(&mut new_board);
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
    let new_hash = hash_board(&new_board);
    memo.insert(hash, new_hash);
    new_board
}
fn tilt_west(board: &Vec<Vec<char>>, memo: &mut HashMap<String, String>) -> Vec<Vec<char>> {
    // right to left
    let rows = board.len();
    let cols = board[0].len();
    let hash = hash_board(board);
    if memo.contains_key(&hash) {
        let hashed_board = memo.get(&hash).unwrap();
        return unhash_board(hashed_board, rows, cols);
    }
    let mut new_board = board.clone(); // should be deep copy
    empty_rocks(&mut new_board);
    let mut moving_rocks = vec![0; rows];
    for c in (0..cols).rev() {
        // println!("{} {:?}", r, moving_rocks);
        for r in 0..rows {
            match board[r][c] {
                '#' => {
                    let mut to_place = moving_rocks[r];
                    let mut cright = c + 1;
                    while to_place > 0 && cright < cols {
                        new_board[r][cright] = 'O';
                        to_place -= 1;
                        cright += 1;
                    }
                    moving_rocks[r] = 0;
                }
                'O' => moving_rocks[r] += 1,
                _ => {
                    // println!("ignoring {}", board[r][c])
                }
            }
        }
    }
    for r in 0..rows {
        let mut to_place = moving_rocks[r];
        let mut cright = 0;
        while to_place > 0 && cright < cols {
            new_board[r][cright] = 'O';
            to_place -= 1;
            cright += 1;
        }
        moving_rocks[r] = 0;
    }
    let new_hash = hash_board(&new_board);
    memo.insert(hash, new_hash);
    new_board
}
fn tilt_south(board: &Vec<Vec<char>>, memo: &mut HashMap<String, String>) -> Vec<Vec<char>> {
    // top to bottom
    let rows = board.len();
    let cols = board[0].len();
    let hash = hash_board(board);
    if memo.contains_key(&hash) {
        let hashed_board = memo.get(&hash).unwrap();
        return unhash_board(hashed_board, rows, cols);
    }
    let mut new_board = board.clone(); // should be deep copy
    empty_rocks(&mut new_board);
    let mut moving_rocks = vec![0; cols];
    for r in 0..rows {
        // println!("{} {:?}", r, moving_rocks);
        for c in 0..cols {
            match board[r][c] {
                '#' => {
                    let mut to_place = moving_rocks[c];
                    let mut rabove = r;
                    while to_place > 0 && rabove > 0 {
                        new_board[rabove - 1][c] = 'O';
                        to_place -= 1;
                        rabove -= 1;
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
        let mut rabove = rows;
        while to_place > 0 && rabove > 0 {
            new_board[rabove - 1][c] = 'O';
            to_place -= 1;
            rabove -= 1;
        }
        moving_rocks[c] = 0;
    }
    let new_hash = hash_board(&new_board);
    memo.insert(hash, new_hash);
    new_board
}
fn tilt_east(board: &Vec<Vec<char>>, memo: &mut HashMap<String, String>) -> Vec<Vec<char>> {
    // left to right
    let rows = board.len();
    let cols = board[0].len();
    let hash = hash_board(board);
    if memo.contains_key(&hash) {
        let hashed_board = memo.get(&hash).unwrap();
        return unhash_board(hashed_board, rows, cols);
    }
    let mut new_board = board.clone(); // should be deep copy
    empty_rocks(&mut new_board);
    let mut moving_rocks = vec![0; rows];
    for c in 0..cols {
        // println!("{} {:?}", r, moving_rocks);
        for r in 0..rows {
            match board[r][c] {
                '#' => {
                    let mut to_place = moving_rocks[r];
                    let mut cleft = c;
                    while to_place > 0 && cleft > 0 {
                        new_board[r][cleft - 1] = 'O';
                        to_place -= 1;
                        cleft -= 1;
                    }
                    moving_rocks[r] = 0;
                }
                'O' => moving_rocks[r] += 1,
                _ => {
                    // println!("ignoring {}", board[r][c])
                }
            }
        }
    }
    for r in 0..rows {
        let mut to_place = moving_rocks[r];
        let mut cleft = cols;
        while to_place > 0 && cleft > 0 {
            new_board[r][cleft - 1] = 'O';
            to_place -= 1;
            cleft -= 1;
        }
        moving_rocks[r] = 0;
    }
    let new_hash = hash_board(&new_board);
    memo.insert(hash, new_hash);
    new_board
}
fn cycle(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut nmap = HashMap::new();
    let mut wmap = HashMap::new();
    let mut smap = HashMap::new();
    let mut emap = HashMap::new();
    let mut n = tilt_north(board, &mut nmap);
    n = tilt_west(&n, &mut wmap);
    n = tilt_south(&n, &mut smap);
    n = tilt_east(&n, &mut emap);
    n
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { board } = input;
    // let mut n = cycle(board);
    // println!("one cycle:");
    // print_board(&n);
    // n = cycle(&n);
    // println!("two cycle");
    // print_board(&n);
    let mut n = board.clone();
    let amount_cycles = 1000000000;
    let mut encountered = HashMap::new();
    let mut curr = 0;
    while curr < amount_cycles {
        let before_hash = hash_board(&n);
        if encountered.contains_key(&before_hash) {
            let cycle_dist = curr - encountered.get(&before_hash).unwrap();
            while curr + cycle_dist < amount_cycles {
                curr += cycle_dist;
            }
        } else {
            encountered.insert(before_hash.clone(), curr);
        }

        n = cycle(&n);
        let after_hash = hash_board(&n);
        if before_hash == after_hash {
            break; //stale now
        }
        curr += 1;
        // if i < 3 {
        //     println!("{}", i);
        //     print_board(&n);
        //     println!();
        // }
    }
    Ok(Answer::Num(compute_board_load(&n) as i128))
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
        assert_eq!(answer, Answer::Num(108826));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(64));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(99291));
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
