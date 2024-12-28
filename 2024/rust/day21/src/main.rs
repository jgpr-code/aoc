#![feature(test)]
extern crate test;

use anyhow::Result;
use common::{
    point::{point, Point},
    Answer,
};
use std::{
    collections::{HashMap, HashSet},
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
    position: Point,
    keypad_type: KeypadType,
}
impl Keypad {
    fn shortest_word_sequences(&self, start: char, word: &str) -> HashSet<String> {
        if word.len() == 0 {
            return HashSet::from(["".to_string()]);
        }
        let to = word.chars().nth(0).unwrap();
        let mut possibilities = HashSet::new();
        let paths = self.shortest_move_sequences(start, to);
        let remaining_possibilities = self.shortest_word_sequences(to, &word[1..]);
        for path in paths {
            for remaining_possibility in remaining_possibilities.iter() {
                possibilities.insert(format!("{}A{}", path, remaining_possibility));
            }
        }
        possibilities
    }
    fn shortest_move_sequences(&self, from: char, to: char) -> HashSet<String> {
        // insight: just picking one sequence is wrong (trial and error approach)
        // for example: directional keypad is at '<' and numeric keypad is at '5'
        // to go from '5' to '1' we would use either '<v' or 'v<', but because the
        // directional keypad starts at '<' the sequence '<v' is better!
        // but does this really matter (seems like it doesn't OR IT DOES):
        // e.g. typing 53 requires <^^A|^^<A >vA|v>A
        // >vA requires vA<A>^A|vA<A^>A
        // v>A requires v<A>A^A

        // assumption: extra turns should be avoided, because they get expensive on the next keypad => probably this is wrong also
        if from == to {
            return HashSet::from(["".to_string()]);
        }
        let from_position = self.keys[&from];
        let to_position = self.keys[&to];
        let delta = to_position - from_position;
        let cx = if delta.x >= 0 { '>' } else { '<' };
        let nx = i128::abs(delta.x) as usize;
        let sx = cx.to_string().repeat(nx);
        let cy = if delta.y >= 0 { 'v' } else { '^' };
        let ny = i128::abs(delta.y) as usize;
        let sy = cy.to_string().repeat(ny);
        let mut possibilities = HashSet::new();
        match self.keypad_type {
            // 789
            // 456
            // 123
            //  0A
            KeypadType::Numeric => match from {
                '7' | '4' | '1' if to == '0' || to == 'A' => {
                    // change x then y
                    possibilities.insert(format!("{sx}{sy}"));
                }
                '0' | 'A' if to == '7' || to == '4' || to == '1' => {
                    // change y then x
                    possibilities.insert(format!("{sy}{sx}"));
                }
                _ => {
                    // either works
                    possibilities.insert(format!("{sx}{sy}"));
                    possibilities.insert(format!("{sy}{sx}"));
                }
            },
            //  ^A
            // <v>
            KeypadType::Directional => match from {
                '<' => {
                    // change x then y
                    possibilities.insert(format!("{sx}{sy}"));
                }
                '^' | 'A' if to == '<' => {
                    // change y then x
                    possibilities.insert(format!("{sy}{sx}"));
                }
                _ => {
                    // either works
                    possibilities.insert(format!("{sx}{sy}"));
                    possibilities.insert(format!("{sy}{sx}"));
                }
            },
        }
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
        let position = keys[&'A'];
        let keypad_type = KeypadType::Numeric;
        Self {
            keys,
            position,
            keypad_type,
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
        let position = keys[&'A'];
        let keypad_type = KeypadType::Directional;
        Self {
            keys,
            position,
            keypad_type,
        }
    }
}

fn code_value(code: &str) -> i128 {
    i128::from_str_radix(&code[0..3], 10).unwrap_or(0)
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { codes } = input;
    let numeric_pad = Keypad::new_numeric();
    let directional_pad = Keypad::new_directional();
    let mut sum = 0;
    for code in codes {
        let mut minimal_length = usize::MAX;
        let typed_by_first = numeric_pad.shortest_word_sequences('A', code);
        for tf in typed_by_first.iter() {
            let typed_by_second = directional_pad.shortest_word_sequences('A', tf);
            for ts in typed_by_second.iter() {
                let typed_by_me = directional_pad.shortest_word_sequences('A', ts);
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
    // third answer 134121 => wrong (still too high)
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
