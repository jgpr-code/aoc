#![feature(test)]
extern crate test;

use anyhow::Result;
use common::{
    point::{point, Point},
    Answer,
};
use std::{collections::HashMap, io, time::Instant};

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
    codes: Vec<String>,
}

fn parse_input(input: &str) -> Result<Input> {
    let codes = input
        .trim()
        .lines()
        .map(|line| String::from(line))
        .collect();
    Ok(Input { codes })
}

#[derive(PartialEq, Eq)]
enum KeypadType {
    Numeric,
    Directional,
}
struct Keypad {
    keys: HashMap<char, Point>,
    keypad_type: KeypadType,
    cache: HashMap<(char, String), String>,
    cache2: HashMap<(String, usize), i128>,
}
impl Keypad {
    fn shortest_word_sequence(&mut self, start: char, word: &str) -> String {
        if word.len() == 0 {
            return String::from("");
        }
        if let Some(sequence) = self.cache.get(&(start, word.to_string())) {
            return sequence.clone();
        }
        let to = word.chars().nth(0).unwrap();
        let move_sequence = self.shortest_move_sequence(start, to);
        let remaining_sequence = self.shortest_word_sequence(to, &word[1..]);
        let sequence = format!("{}A{}", move_sequence, remaining_sequence);
        self.cache
            .insert((start, word.to_string()), sequence.clone());
        sequence
    }
    fn len_shortest_word_sequence(&mut self, word: &str, depth: usize) -> i128 {
        assert!(self.keypad_type == KeypadType::Directional);
        // e.g.
        // d1 start = A, word = >^A => move(A>) A + move(>^) A + move(^A) A,
        // start is always 'A' because at previous depth we must push A to enter the char after a move sequence
        if let Some(len) = self.cache2.get(&(String::from(word), depth)) {
            return *len;
        }
        let mut word_vec = vec!['A'];
        word_vec.extend(word.chars());
        let mut len = 0i128;
        for w in word_vec.windows(2) {
            let mut next_word = self.shortest_move_sequence(w[0], w[1]);
            next_word.push('A');
            if depth == 0 {
                len += next_word.len() as i128;
            } else {
                len += self.len_shortest_word_sequence(&next_word, depth - 1);
            }
        }
        self.cache2.insert((String::from(word), depth), len);
        len
    }
    fn shortest_move_sequence(&self, from: char, to: char) -> String {
        let from_position = self.keys[&from];
        let to_position = self.keys[&to];
        let delta = to_position - from_position;
        let cx = if delta.x >= 0 { '>' } else { '<' };
        let nx = i128::abs(delta.x) as usize;
        let sx = cx.to_string().repeat(nx);
        let cy = if delta.y >= 0 { 'v' } else { '^' };
        let ny = i128::abs(delta.y) as usize;
        let sy = cy.to_string().repeat(ny);
        match self.keypad_type {
            // 789
            // 456
            // 123
            //  0A
            KeypadType::Numeric => match from {
                '7' | '4' | '1' if to == '0' || to == 'A' => {
                    // change x then y
                    return format!("{sx}{sy}");
                }
                '0' | 'A' if to == '7' || to == '4' || to == '1' => {
                    // change y then x
                    return format!("{sy}{sx}");
                }
                _ => {
                    // either works do in this order < ^v >
                    if sx.contains("<") {
                        return format!("{sx}{sy}");
                    } else {
                        return format!("{sy}{sx}");
                    }
                }
            },
            //  ^A
            // <v>
            KeypadType::Directional => match from {
                '<' => {
                    // change x then y
                    return format!("{sx}{sy}");
                }
                '^' | 'A' if to == '<' => {
                    // change y then x
                    return format!("{sy}{sx}");
                }
                _ => {
                    // either works do in this order < ^v >
                    if sx.contains("<") {
                        return format!("{sx}{sy}");
                    } else {
                        return format!("{sy}{sx}");
                    }
                }
            },
        }
    }
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 |>A<|
    //     +---+---+
    fn new_numeric() -> Self {
        #[rustfmt::skip]
        let keys = HashMap::from([
            ('7', point!(0, 0)), ('8', point!(1, 0)), ('9', point!(2, 0)),
            ('4', point!(0, 1)), ('5', point!(1, 1)), ('6', point!(2, 1)),
            ('1', point!(0, 2)), ('2', point!(1, 2)), ('3', point!(2, 2)),
                                 ('0', point!(1, 3)), ('A', point!(2, 3)),
        ]);
        let keypad_type = KeypadType::Numeric;
        Self {
            keys,
            keypad_type,
            cache: HashMap::new(),
            cache2: HashMap::new(),
        }
    }
    //     +---+---+
    //     | ^ |>A<|
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    fn new_directional() -> Self {
        #[rustfmt::skip]
        let keys = HashMap::from([
                                 ('^', point!(1, 0)), ('A', point!(2, 0)),
            ('<', point!(0, 1)), ('v', point!(1, 1)), ('>', point!(2, 1)),
        ]);
        let keypad_type = KeypadType::Directional;
        Self {
            keys,
            keypad_type,
            cache: HashMap::new(),
            cache2: HashMap::new(),
        }
    }
}

fn code_value(code: &str) -> i128 {
    i128::from_str_radix(&code[0..3], 10).unwrap_or(0)
}

fn code_complexity(code: &str, robots: usize) -> i128 {
    let mut numeric_pad = Keypad::new_numeric();
    let mut directional_pad = Keypad::new_directional();
    let mut start = Instant::now();
    let typed_on_numeric_pad = numeric_pad.shortest_word_sequence('A', code);
    let mut last_stage = typed_on_numeric_pad;
    for stage in 0..robots {
        println!(
            "processing stage: {}, len last_stage: {}, duration last_stage: {:?}",
            stage,
            last_stage.len(),
            start.elapsed(),
        );
        start = Instant::now();
        last_stage = directional_pad.shortest_word_sequence('A', &last_stage);
    }
    let mut minimal_length = usize::MAX;
    minimal_length = usize::min(minimal_length, last_stage.len());
    minimal_length as i128 * code_value(code)
}

fn dfs_code_complexity(code: &str, robots: usize) -> i128 {
    let mut numeric_pad = Keypad::new_numeric();
    let mut directional_pad = Keypad::new_directional();
    let typed_on_numeric_pad = numeric_pad.shortest_word_sequence('A', code);
    let len = directional_pad.len_shortest_word_sequence(&typed_on_numeric_pad, robots - 1);
    len * code_value(code)
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { codes } = input;
    let mut sum = 0;
    for code in codes {
        sum += code_complexity(code, 2);
    }
    Ok(Answer::Num(sum))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { codes } = input;
    let mut sum = 0;
    for code in codes {
        sum += dfs_code_complexity(code, 25);
    }
    Ok(Answer::Num(sum))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day21_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(126384));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(134120));
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
        assert_eq!(answer, Answer::Num(154115708116294));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(167389793580400));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
