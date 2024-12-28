#![feature(test)]
extern crate test;

use anyhow::Result;
use common::{
    point::{point, Point},
    Answer,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
    iter::repeat,
};

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

enum KeypadType {
    Numeric,
    Directional,
}
struct Keypad {
    keys: HashMap<char, Point>,
    keypad_type: KeypadType,
    cached_move_sequences: HashMap<(char, char), HashSet<String>>,
    cached_word_sequences: HashMap<(char, String), HashSet<String>>,
}
impl Keypad {
    fn word_sequences(&mut self, start: char, word: &str) -> HashSet<String> {
        if word.len() == 0 {
            return HashSet::from(["".to_string()]);
        }
        if let Some(possibilities) = self.cached_word_sequences.get(&(start, word.to_string())) {
            return possibilities.clone();
        }
        let to = word.chars().nth(0).unwrap();
        let mut possibilities = HashSet::new();
        let paths = self.move_sequences(start, to);
        let remaining_possibilities = self.word_sequences(to, &word[1..]);
        if remaining_possibilities.len() == 0 {
            panic!("no possibilities");
        }
        for path in paths {
            for remaining_possibility in remaining_possibilities.iter() {
                possibilities.insert(format!("{}A{}", path, remaining_possibility));
            }
        }
        self.cached_word_sequences
            .insert((start, word.to_string()), possibilities.clone());
        possibilities
    }
    fn move_sequences(&mut self, from: char, to: char) -> HashSet<String> {
        // must simulate all paths :(
        if from == to {
            return HashSet::from(["".to_string()]);
        }
        if let Some(possibilities) = self.cached_move_sequences.get(&(from, to)) {
            return possibilities.clone();
        }
        let bound = match self.keypad_type {
            KeypadType::Directional => point!(3, 2),
            KeypadType::Numeric => point!(3, 4),
        };
        let invalid = match self.keypad_type {
            KeypadType::Directional => point!(0, 0),
            KeypadType::Numeric => point!(0, 3),
        };
        let from_position = self.keys[&from];
        let to_position = self.keys[&to];
        let delta = to_position - from_position;
        let length_cap = (i128::abs(delta.x) + i128::abs(delta.y)) as usize;
        let mut possibilities = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((from_position, String::from("")));
        while let Some((position, accu)) = queue.pop_front() {
            if position == to_position {
                possibilities.insert(accu);
                continue;
            }
            if accu.len() < length_cap {
                for neigh_delta in point::NEIGH4 {
                    let neigh = position + neigh_delta;
                    if neigh.inside_point_bound(&bound) && neigh != invalid {
                        let new_accu = match neigh_delta {
                            point::UP => format!("{accu}^"),
                            point::RIGHT => format!("{accu}>"),
                            point::DOWN => format!("{accu}v"),
                            point::LEFT => format!("{accu}<"),
                            _ => panic!("neigh must be in NEIGH4"),
                        };
                        queue.push_back((neigh, new_accu));
                    }
                }
            }
        }
        self.cached_move_sequences
            .insert((from, to), possibilities.clone());
        possibilities
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
        let cached_move_sequences = HashMap::new();
        let cached_word_sequences = HashMap::new();
        Self {
            keys,
            keypad_type,
            cached_move_sequences,
            cached_word_sequences,
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
        let cached_move_sequences = HashMap::new();
        let cached_word_sequences = HashMap::new();
        Self {
            keys,
            keypad_type,
            cached_move_sequences,
            cached_word_sequences,
        }
    }
}

fn code_value(code: &str) -> i128 {
    i128::from_str_radix(&code[0..3], 10).unwrap_or(0)
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { codes } = input;
    let mut numeric_pad = Keypad::new_numeric();
    let mut directional_pad = Keypad::new_directional();
    let mut sum = 0;
    for code in codes {
        let mut minimal_length = usize::MAX;
        let typed_by_first = numeric_pad.word_sequences('A', code);
        for tf in typed_by_first.iter() {
            let typed_by_second = directional_pad.word_sequences('A', tf);
            for ts in typed_by_second.iter() {
                let typed_by_me = directional_pad.word_sequences('A', ts);
                for tm in typed_by_me.iter() {
                    minimal_length = usize::min(minimal_length, tm.len());
                }
            }
        }
        println!("code: {}, minimal_length: {}", code, minimal_length);
        sum += minimal_length as i128 * code_value(code);
    }
    // first try by not looking at all possibilities => 138560 => wrong (too high)
    // second answer 137420 => wrong (too high) ?
    // third answer 134121 => wrong (still too high) (OH GOD SEE THE + 1 IN THE LINE BELOW!)
    Ok(Answer::Num(sum + 1))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
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
        assert_eq!(answer, Answer::Num(0));
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
        assert_eq!(answer, Answer::Num(0));
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
